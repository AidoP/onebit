# One Bit
### Scenario
Our agents have discovered an enemy satellite in orbit carrying sensitive data as well as a copy of the program with all symbols included. Enemy agents send a code to the satellite and receive some super secret info in return.

Using a powerful laser we are able to re-program the satellite as it orbits, however, due to cosmic radiation redundancies the satellite has systems on-board which prevent us from changing more than a single bit before the working memory is restored and the system is rebooted.

Our agents discovered the following about the system:
- It uses a RISCV processor utilising the RV32I base instruction set
- Writing to address 0 causes the system to reset
- The input unit is memory-mapped to address 1
- The output unit is memory-mapped to address 2
- A non-deterministic one time code generator is memory-mapped to addresses 10-73, returning a unique 64 byte code

### Usage
To use the laser and interact with the satellite, make a get request for `/?byte=<byte to change>&bit=<bit to change within byte>&input=<the code sent to the satellite>`. For example using curl:

```sh
curl "<address>:8000/?byte=133&bit=7&input=CodeWeCantGuess"
```

You can also check the default response by leaving out the `byte` and `bit` paramters.

Students should be provided with `encode.elf` for reverse engineering which contains a placeholder flag.

# Building
### Dependencies
To build, `llvm` and `lld` are required. Rust nightly is required for the virtual machine.

The RISCV program is built by executing `cd program && ./build` then the virtual machine can be built and run with cargo using `cd ../vm && cargo run`. The `vm` directory may need to be set to Rust nightly with rustup using `rustup override set nightly` while in the directory.