use super::super::syscall_dispatch_aarch64;
pub unsafe fn InitSingleton() {
}

pub fn init() {
    // TODO set up GIC
}

use core::arch::asm;

#[repr(C)]
pub struct ExceptionStateEL1{
    pub spsr_el1: u64,
    pub elr_el1: u64,
    pub esr_el1: u64,
    pub tpidr_el0: u64,
    pub x: [u64;32]
}


pub struct EsrDefs{}

impl EsrDefs{
    pub const ESR_IL:u64            = 1 << 25;
    pub const ESR_EC_SHIFT:u64      = 26;
    pub const ESR_EC_MASK:u64       = 0x3f << 26;
    pub const EC_UNKNOWN:u64        = 0x00;    /* Unkwn exception */
    pub const EC_FP_SIMD:u64        = 0x07;    /* FP/SIMD trap */
    pub const EC_BRANCH_TGT:u64     = 0x0d;    /* Branch target exception */
    pub const EC_ILL_STATE:u64      = 0x0e;    /* Illegal execution state */
    pub const EC_SVC:u64            = 0x15;    /* SVC trap */
    pub const EC_MSR:u64            = 0x18;    /* MSR/MRS trap */
    pub const EC_FPAC:u64           = 0x1c;    /* Faulting PAC trap */
    pub const EC_INSN_ABORT_L:u64   = 0x20;    /* Instruction abort, from lower EL */
    pub const EC_INSN_ABORT:u64     = 0x21;    /* Instruction abort, from same EL */ 
    pub const EC_PC_ALIGN:u64       = 0x22;    /* PC alignment fault */
    pub const EC_DATA_ABORT_L:u64   = 0x24;    /* Data abort, from lower EL */
    pub const EC_DATA_ABORT:u64     = 0x25;    /* Data abort, from same EL */ 
    pub const EC_SP_ALIGN:u64       = 0x26;    /* SP alignment fault */
    pub const EC_TRAP_FP:u64        = 0x2c;    /* Trapped FP exception */
    pub const EC_SERROR:u64         = 0x2f;    /* SError interrupt */
    pub const EC_SOFTSTP_EL0:u64    = 0x32;    /* Software Step, from lower EL */
    pub const EC_SOFTSTP_EL1:u64    = 0x33;    /* Software Step, from same EL */
    pub const EC_WATCHPT_EL1:u64    = 0x35;    /* Watchpoint, from same EL */
    pub const EC_BRK:u64            = 0x3c;    /* Breakpoint */
    pub fn GetExceptionFromESR(esr:u64) -> u64{
        return (esr & EsrDefs::ESR_EC_MASK) >> EsrDefs::ESR_EC_SHIFT;
    }
}

pub fn GetEsrEL1() -> u64 {
    unsafe {
        let value:u64;
        asm!(
            "mrs  {}, esr_el1",
            out(reg) value,
           );
        return value;
    }
}

pub fn GetException() -> u64{
    let esr:u64 = GetEsrEL1();
    return EsrDefs::GetExceptionFromESR(esr);
}

#[no_mangle]
pub extern "C" fn exception_handler_unhandled(_exception_frame_ptr:usize){
    // MUST CHECK ptr!=0 before dereferencing
    // ptr == 0 indicates an empty entry in the exception table,
    // in which case the context won't be saved/restored by the wrapper
    // and this function MUST NOT return
    panic!("unhandled exception");
}

// NOT IMPLEMENTED:  
// 1. We don't use SP_EL0 in kernel space
//    exception_handler_el1t_{sync,irq,fiq,serror}
//
// 2. We don't handle 32 bit process state
//    exception_handler_el0_32_{sync,irq,fiq,serror}

// TODO add parameter to the handler
// TODO implement them
#[no_mangle]
pub extern "C" fn exception_handler_el1h_sync(exception_frame_ptr:usize){
    return exception_handler_unhandled(exception_frame_ptr);
}
#[no_mangle]
pub extern "C" fn exception_handler_el1h_irq(exception_frame_ptr:usize){
    return exception_handler_unhandled(exception_frame_ptr);
}
#[no_mangle]
pub extern "C" fn exception_handler_el1h_fiq(exception_frame_ptr:usize){
    return exception_handler_unhandled(exception_frame_ptr);
}
#[no_mangle]
pub extern "C" fn exception_handler_el1h_serror(exception_frame_ptr:usize){
    return exception_handler_unhandled(exception_frame_ptr);
}

// TODO add parameter to the handler
// TODO implement el0_64_sync handler for syscalls
// read saved x0 from the frame exception_frame_ptr[+something]
#[no_mangle]
pub extern "C" fn exception_handler_el0_sync(exception_frame_ptr:usize){
    let ec = GetException();
    if exception_frame_ptr == 0 {
        panic!("exception frame is null pointer\n")
    }
    match ec {
        EsrDefs::EC_SVC => {
            // arm64 linux syscall calling convention
            // TODO maybe there is a better/safer way of this pointer cast
            let ctx_p = exception_frame_ptr as *mut ExceptionStateEL1;
            let ctx_p = ctx_p.cast::<ExceptionStateEL1>();
            let ctx = unsafe { &mut *ctx_p };
            // syscall number from w8
            let call_no = ctx.x[8] as u32;
            let arg0 = ctx.x[0];
            let arg1 = ctx.x[1];
            let arg2 = ctx.x[2];
            let arg3 = ctx.x[3];
            let arg4 = ctx.x[4];
            let arg5 = ctx.x[5];
            // write syscall ret to x0
            ctx.x[0] = syscall_dispatch_aarch64(call_no,arg0,arg1,arg2,arg3,arg4,arg5);
            // TODO do we need to write the "second ret val" back to x1?
        },
        _ => {
            panic!("unhandled sync exception from el0: {}\n", ec);
        }
    }
}

#[no_mangle]
pub extern "C" fn exception_handler_el0_irq(){return;}
#[no_mangle]
pub extern "C" fn exception_handler_el0_fiq(){return;}
#[no_mangle]
pub extern "C" fn exception_handler_el0_serror(){return;}
