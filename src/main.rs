// src/main.rs
#![no_std]
#![no_main]

pub mod drivers;
pub mod experiments;
pub mod memory;
pub mod set_macros;
pub mod task;

extern crate alloc;

use task::cpu_info::cpuinfo::get_info_cpu;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

use linked_list_allocator::LockedHeap;
//use crate::drivers::vga::xd8000::{println, text::text_write};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Runix Kernel v0.1.0");
    println_error!(
        ""
    );
    println_warn!("3");
    println!("4");
    println!("5");
    println!("6");
    println!("7");
    println!("8");
    println!("9");
    println!("10");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
