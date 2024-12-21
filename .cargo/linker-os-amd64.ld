/* OUTPUT_FORMAT("binary") */
/* ENTRY(_start) */

/* SECTIONS { */
/*     /1* This is likely NOT 0x7C00 on OS, just bootloader *1/ */
/*     . = 0x7C00; */
/*     .multiboot : { */
/*         . = ALIGN(4);        /1* Ensure it's 4-byte aligned *1/ */
/*         .long 0x1BADB002;    /1* Magic number *1/ */
/*         .long 0;             /1* Flags (0, meaning no special flags) *1/ */
/*         .long kernel_size;   /1* Kernel size (optional, calculated later) *1/ */
/*         .long 0;             /1* Checksum (optional) *1/ */
/*     } */
/*     .text : { *(.text) } */
/*     .data : { *(.data) } */
/*     .bss : { *(.bss) } */
/* } */
