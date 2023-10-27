// VBAR should be set to &vector_table during kernel setup
.globl vector_table
// handlers defined in exception.rs
.extern exception_handler_unhandled

.extern exception_handler_el0_sync
.extern exception_handler_el0_irq
.extern exception_handler_el0_fiq
.extern exception_handler_el0_serror

.extern exception_handler_el1h_sync
.extern exception_handler_el1h_irq
.extern exception_handler_el1h_fiq
.extern exception_handler_el1h_serror

.macro el0_sync_save_regs
    stp x30, x30, [sp, #-16]!
    stp x28, x29, [sp, #-16]!
    stp x26, x27, [sp, #-16]!
    stp x24, x25, [sp, #-16]!
    stp x22, x23, [sp, #-16]!
    stp x20, x21, [sp, #-16]!
    stp x18, x19, [sp, #-16]!
    stp x16, x17, [sp, #-16]!
    stp x14, x15, [sp, #-16]!
    stp x12, x13, [sp, #-16]!
    stp x10, x11, [sp, #-16]!
    stp x7, x9, [sp, #-16]!
    stp x6, x7, [sp, #-16]!
    stp x4, x5, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
    stp x0, x1, [sp, #-16]!
    // tpidr_el0, esr_el1
    mrs x9, tpidr_el0
    mrs x10, esr_el1
    stp x9, x10, [sp, #-16]
    // elr_el1, spsr_el1
    mrs x11, elr_el1;
    mrs x12, spsr_el1;
    stp x11, x12, [sp, #-16]!
.endm

.macro el0_sync_restore_regs
    ldp x11, x12, [sp], #16
    msr elr_el1, x11
    msr spsr_el1, x12
    ldp x9, x10, [sp], #16
    msr tpidr_el0, x9
    msr esr_el1, x10
    ldp x0, x1, [sp], #16
    ldp x2, x1, [sp], #16
    ldp x4, x1, [sp], #16
    ldp x6, x1, [sp], #16
    ldp x8, x1, [sp], #16
    ldp x10, x1, [sp], #16
    ldp x12, x1, [sp], #16
    ldp x14, x1, [sp], #16
    ldp x16, x1, [sp], #16
    ldp x18, x1, [sp], #16
    ldp x20, x1, [sp], #16
    ldp x22, x1, [sp], #16
    ldp x24, x1, [sp], #16
    ldp x26, x1, [sp], #16
    ldp x28, x1, [sp], #16
    ldp x30, xzr, [sp], #16
.endm

// mitigaton of specter bhi see
// TODO insert mitigation to the handler flow if the exception is taken
// from EL0
// https://documentation-service.arm.com/static/623c60d13b9f553dde8fd8e6?token=
.macro spectre_bhb_loop cnt
    mov x0, #\cnt
1:
    b pc + 4
    subs x18, x18, #1
    bne 1b
    dsb nsh
    isb
.endm

enter_el1h_sync:
    el0_sync_save_regs
    mov x0, sp
    bl exception_handler_el1h_sync
    el0_sync_restore_regs
    eret

enter_el1h_irq:
    el0_sync_save_regs
    mov x0, sp
    bl exception_handler_el1h_irq
    el0_sync_restore_regs
    eret

enter_el1h_fiq:
    el0_sync_save_regs
    mov x0, sp
    bl exception_handler_el1h_fiq
    el0_sync_restore_regs
    eret

enter_el1h_serror:
    el0_sync_save_regs
    mov x0, sp
    bl exception_handler_el1h_serror
    el0_sync_restore_regs
    eret

enter_el0_sync:
    el0_sync_save_regs
    mov x0, sp
    bl exception_handler_el0_sync
    el0_sync_restore_regs
    eret

enter_el0_irq:
    el0_sync_save_regs
    mov x0, sp
    bl exception_handler_el0_irq
    el0_sync_restore_regs
    eret

enter_el0_fiq:
    el0_sync_save_regs
    mov x0, sp
    bl exception_handler_el0_fiq
    el0_sync_restore_regs
    eret

enter_el0_serror:
    el0_sync_save_regs
    mov x0, sp
    bl exception_handler_el0_serror
    el0_sync_restore_regs
    eret


// this should be more sophisticated e.g. causing
// exception with brk. But for now we simply cause a panic
// without saving/restoring the registers
.macro v_empty elx
.align 7
    mov x0, #0
    bl exception_handler_unhandled
    eret
.endm

.macro v_entry elx handler
.align 7
    b \handler
.endm



.align 11
.globl vector_table
.type vector_table STT_FUNC
vector_table:
//          ELx    Handler
v_empty     1      // we don't use SP_EL0 in EL1
v_empty     1
v_empty     1
v_empty     1

// interrupts are currently masked kernel
v_entry     1      enter_el1h_sync
v_empty     1      // TODO add entries
v_empty     1      
v_empty     1      

v_entry     0      enter_el0_sync
v_entry     0      enter_el0_irq
v_entry     0      enter_el0_fiq
v_entry     0      enter_el0_serror

// 32bit state is not used
v_empty     0
v_empty     0
v_empty     0
v_empty     0

// END exception vector table
