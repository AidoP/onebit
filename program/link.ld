ENTRY(entry)

MEMORY
{
    ROM (rx) : ORIGIN = 0x1000, LENGTH = 60k
    RAM (rwx) : ORIGIN = 0x800000, LENGTH = 1024
}

SECTIONS
{
    .text : {
        * (.text*);
    } > ROM
    .rodata : {
        * (.rodata*);
    } > ROM
}

RAM_START = 0x800000;
RANDOM_CODE = 0x10;
