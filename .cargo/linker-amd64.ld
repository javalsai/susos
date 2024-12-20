OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS {
    . = 0x7C00;
    .multiboot : {
        . = ALIGN(4);        /* Ensure it's 4-byte aligned */
        .long 0x1BADB002;    /* Magic number */
        .long 0;             /* Flags (0, meaning no special flags) */
        .long kernel_size;   /* Kernel size (optional, calculated later) */
        .long 0;             /* Checksum (optional) */
    }
    .text : { *(.text) }
    .data : { *(.data) }
    .bss : { *(.bss) }
}
