
use spin::Mutex;

pub struct CPUContext {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,

    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,

    pub rip: u64,
    pub rsp: u64,

    pub cs: u64,
    pub ss: u64,
    pub ds: u64,
    pub es: u64,
    pub fs: u64,
    pub gs: u64,

    pub rflags: u64,

    pub cr3: u64,
}

pub struct Task{
    pub id: usize,
    pub name: &'static str,
    pub priority: u8,
    pub context: CPUContext,
}

pub struct TaskManager {
}

fn task() -> ! {
    loop{

    }
}