mod gdt;
mod idt;
#[cfg(feature = "ioport_bitmap")]
pub mod ioport;
mod syscall;
mod trap;

pub use trap::TrapFrame;

/// Initialize interrupt handling on x86_64.
///
/// # Safety
///
/// This function will:
///
/// - Disable interrupt.
/// - Switch to a new [GDT], extend 7 more entries from the current one.
/// - Switch to a new [TSS], set `GSBASE` to its base address.
/// - Switch to a new [IDT], override the current one.
/// - Enable [`syscall`] instruction.
///     - set `EFER::SYSTEM_CALL_EXTENSIONS`
///
/// [GDT]: https://wiki.osdev.org/GDT
/// [IDT]: https://wiki.osdev.org/IDT
/// [TSS]: https://wiki.osdev.org/Task_State_Segment
/// [`syscall`]: https://www.felixcloutier.com/x86/syscall
///
pub unsafe fn init() {
    use log::info;
    info!("Initializing trapframe...");

    x86_64::instructions::interrupts::disable();
    gdt::init();
    info!("GDT initialization completed");
    idt::init();
    info!("IDT initialization completed");
    syscall::init();
    info!("Syscall related register initialization completed");
}

/// User space context
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct UserContext {
    pub general: GeneralRegs,
    pub trap_num: usize,
    pub error_code: usize,
}

/// General registers
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct GeneralRegs {
    pub rax: usize,
    pub rbx: usize,
    pub rcx: usize,
    pub rdx: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rbp: usize,
    pub rsp: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub rip: usize,
    pub rflags: usize,
    pub fsbase: usize,
    pub gsbase: usize,
}

unsafe impl pod::Pod for GeneralRegs {}
unsafe impl pod::Pod for UserContext {}

impl UserContext {
    /// Get number of syscall
    pub fn get_syscall_num(&self) -> usize {
        self.general.rax
    }

    /// Get return value of syscall
    pub fn get_syscall_ret(&self) -> usize {
        self.general.rax
    }

    /// Set return value of syscall
    pub fn set_syscall_ret(&mut self, ret: usize) {
        self.general.rax = ret;
    }

    /// Get syscall args
    pub fn get_syscall_args(&self) -> [usize; 6] {
        [
            self.general.rdi,
            self.general.rsi,
            self.general.rdx,
            self.general.r10,
            self.general.r8,
            self.general.r9,
        ]
    }

    /// Set instruction pointer
    pub fn set_ip(&mut self, ip: usize) {
        self.general.rip = ip;
    }

    /// Set stack pointer
    pub fn set_sp(&mut self, sp: usize) {
        self.general.rsp = sp;
    }

    /// Get stack pointer
    pub fn get_sp(&self) -> usize {
        self.general.rsp
    }

    /// Set tls pointer
    pub fn set_tls(&mut self, tls: usize) {
        self.general.fsbase = tls;
    }
}
