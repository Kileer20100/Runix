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

use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

/// Define the stand and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    
    unsafe {
        use x86_64::instructions::port::Port;
        let mut port = Port::<u8>::new(0xE9);
        
        let msg = b"Runix kernel started!\n";
        for &byte in msg {
            port.write(byte);
        }
    }

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
