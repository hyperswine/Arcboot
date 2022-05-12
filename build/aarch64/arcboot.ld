ENTRY(_setup)

/* 
1. setup sp at at a relatively safe paddr, 0x80000, set it to grow up? Down is fine too but the other segments must be loaded at the proper locations in physical RAM
2. load .text, .data, .rodata, .bss as usual
*/

/* If the system is identity mapped, just load it at 0x80000 anway */
VADDR_LOAD = 0x80000;
PAGE_SIZE_LD = 4K;

/* Physical Addressing Constants */
DRAM_LOAD_ADDR = 0x0;
BINARY_LOAD_ADDR = 0x80000;

/* Ensure boot core stack is set to load */
/* RO = 4, RX = 5, RW = 6, RWX = 7 */
PHDRS
{
    segment_code            PT_LOAD FLAGS(5);
    segment_data            PT_LOAD FLAGS(6);
    segment_boot_core_stack PT_LOAD FLAGS(6);
}

SECTIONS {
    . = BINARY_LOAD_ADDR;

	/* READ ONLY SEGMENTS */
    __code_start = .;
	.text : AT(BINARY_LOAD_ADDR)
    {
        KEEP(*(.text._setup))
        *(.text._start_arguments)
        *(.text._start_rust)
        *(.text*)
    } :segment_code
	.rodata : ALIGN(8) { *(.rodata*) } :segment_code
    /* global offset table */
    .got    : ALIGN(8) { *(.got)     } :segment_code

	. = ALIGN(PAGE_SIZE_LD);
    __code_end_exclusive = .;
    
	/* READ WRITE SEGMENTS */
    __data_start = .;
    .data : { *(.data*) } :segment_data
    .bss (NOLOAD) : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } :segment_data


	/* STACK for the Boot Core */
	. += PAGE_SIZE_LD;

    .boot_core_stack (NOLOAD) : AT(DRAM_LOAD_ADDR)
    {
        __boot_core_stack_start = .;
        . += BINARY_LOAD_ADDR;
        __boot_core_stack_end_exclusive = .;
    } :segment_boot_core_stack
}
