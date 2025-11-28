ENTRY(secret_start)

MEMORY {
    ram (rw) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS {
    .start 0x80000000 : {
        *(.text.secret_start)
    }

    .text : {
        *(.text*)
    } > ram

    .rodata : {
        *(.rodata*)
    } > ram
}
