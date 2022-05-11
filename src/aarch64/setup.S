.global _setup
_setup:
    // core should be in EL2 after firmware has booted
	mrs	x0, CurrentEL
	cmp	x0, {CONST_CURRENTEL_EL2}
	b.ne	.L_parking_loop

    // hook onto the boot core
	mrs	x1, MPIDR_EL1
	and	x1, x1, {CONST_CORE_ID_MASK}
	ldr	x2, BOOT_CORE_ID
	cmp	x1, x2
	b.ne	.L_parking_loop

	// startup dram
	ADR_REL	x0, __bss_start
	ADR_REL x1, __bss_end_exclusive

.L_bss_init_loop:
	cmp	x0, x1
	b.eq	.L_prepare_rust
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop

.L_prepare_rust:
	// stack pointer
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

	b	_start_rust

.L_parking_loop:
	wfe
	b	.L_parking_loop
