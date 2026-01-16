// src/main.rs
#![no_std]
#![no_main]

pub mod drivers;
pub mod experiments;
pub mod memory;
pub mod set_macros;
pub mod task;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

use linked_list_allocator::LockedHeap;
//use crate::drivers::vga::xd8000::{println, text::text_write};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Runix Kernel v0.1.0");
    let mut couter = 0;
    loop {
        println!("\nTick: {}", couter);
        couter += 1;
        for _ in 0..10000 {
            core::hint::spin_loop();
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
