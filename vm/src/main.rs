//! A Rysk fork for simulating a CTF challenge
#![feature(proc_macro_hygiene, decl_macro)]

const RESPONSE_TIME: std::time::Duration = std::time::Duration::from_secs(3);

#[macro_use]
extern crate rocket;
use rocket::{config::{Config, Environment}, request::Request, response::{self,Response,Responder}};

use rysk_core::{Core, Register32, Xlen};

mod memory;

const PATH: &str = "/usr/share/encode.bin";

fn main() {
    let config = Config::build(Environment::Production)
        .port(std::env::var("PORT").unwrap_or("8000".to_string()).parse().unwrap())
        .finalize().unwrap();
    rocket::custom(config).mount("/", routes![index, request, default_request]).launch();
}

#[get("/", rank = 2)]
fn index() -> &'static str {
    include_str!("../../readme.md")
}

#[get("/?<byte>&<bit>&<input>", rank = 0)]
fn request(byte: usize, bit: usize, input: String) -> Result<Vec<u8>, RequestError> {
    use memory::Memory;
    let mut mmu = Memory::new(input.as_bytes());
    let mut core = Core::<Register32>::new(Memory::FIRMWARE_START as u32);

    mmu.load_firmware(PATH).map_err(|_| RequestError::LoadError)?;

    // Flip a bit in the firmware
    if let Some(byte) = mmu.firmware.get_mut(byte) {
        if bit < 8 {
            *byte = *byte ^ (1 << bit);
    
            let start_time = std::time::Instant::now();
            while mmu.shutdown == 0 {
                if core.execute(mmu.fetch(core.pc.unsigned()), &mut mmu).map_err(|err| {eprintln!("{:?}, pc={:X}", err, core.pc.unsigned()); err}).is_err() || (std::time::Instant::now() - start_time) > RESPONSE_TIME {
                    // Ensure 3 seconds pass before returning to prevent brute-forcing
                    std::thread::sleep(RESPONSE_TIME - (std::time::Instant::now() - start_time));
                    return Err(RequestError::Reset)
                }
            }

            // Ensure 3 seconds pass before returning to prevent brute-forcing
            std::thread::sleep(RESPONSE_TIME - (std::time::Instant::now() - start_time));

            Ok(mmu.output)
        } else {
            Err(RequestError::InvalidBits)
        }
    } else {
        Err(RequestError::InvalidBytes)
    }
}

#[get("/?<input>", rank = 1)]
fn default_request(input: String) -> Result<Vec<u8>, RequestError> {
    use memory::Memory;
    let mut mmu = Memory::new(input.as_bytes());
    let mut core = Core::<Register32>::new(Memory::FIRMWARE_START as u32);

    mmu.load_firmware(PATH).map_err(|_| RequestError::LoadError)?;
    while mmu.shutdown == 0 {
        if core.execute(mmu.fetch(core.pc.unsigned()), &mut mmu)
            .map_err(|err| {eprintln!("{:?}, pc={:X}", err, core.pc.unsigned()); err})
            .is_err()
        {
            return Err(RequestError::Reset)
        }
    }

    Ok(mmu.output)
}

#[derive(Debug)]
pub enum RequestError {
    LoadError,
    InvalidBits,
    InvalidBytes,
    Reset,
}
impl<'r> Responder<'r> for RequestError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        use std::io::Cursor;
        let (status, message) = match self {
            Self::LoadError => (500, "Error with CTF challenge: Unable to load RISCV program"),
            Self::InvalidBits => (400, "Invalid bits value. Bits are indexed from 0 and therefore must be in the range 0 to 7"),
            Self::InvalidBytes => (400, "Invalid bytes value; out of range. Byte indices start from 0 for the start of the RISCV program"),
            Self::Reset => (200, "Satellite has reset unexpectedly"),
        };
        Response::build()
            .sized_body(
                Cursor::new(format!("Error with status {status}\n{body}\n",status=status, body=message))
            )
            .raw_status(status, message)
            .ok()
    }
}
