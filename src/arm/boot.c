#include <stdint.h>

static void boot_jump(uint32_t *addr)
{
    if (CONTROL_nPRIV_Msk & __get_CONTROL())
    {
        EnablePrivilegeMode();
    }
    // disable NVIC interrupts (ICER)
    // basically vector stuff
    disable_nvic();

    // disable device-specific peripheral interrupts
    disable_peripherals();

    // clear pending interrupts in NVIC (ICPR)
    disable_nvic_pending();

    // disable systick, clear pending bit
    // (ctl reg and interrupt & state reg)
    SysTick->CTRL = 0;
    SCB->ICSR |= SCB_ICSR_PENDSTCLR_Msk;

    // disable individual fault handlers
    // (system control & state reg)
    SCB->SHCSR &= ~(SCB_SHCSR_USGFAULTENA_Msk | SCB_SHCSR_BUSFAULTENA_Msk | SCB_SHCSR_MEMFAULTENA_Msk);

    // main stack pointer set to psp (process stack pointer)
    activate_msp();

    // load vector table address of the app into VTOR
    // (vector table offset reg)
    load_vtor(addr);

    BootJumpASM(addr[0], addr[1]);
}
