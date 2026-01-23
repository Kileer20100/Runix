// src/main.rs
#![no_std]
#![no_main]

pub mod drivers;
pub mod experiments;
pub mod memory;
pub mod set_macros;
pub mod task;
pub mod cpu;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

use linked_list_allocator::LockedHeap;
use core::panic::PanicInfo;

use crate::cpu::cpu::cpu_info;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    cpu_info();

    //let mut couter = 0;
    loop {
        unsafe {core::arch::asm!("hlt");}
        
        /*
        println!("\nTick: {}", couter);
        couter += 1;
        for _ in 0..1 {
            core::hint::spin_loop();
        }
        */
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
