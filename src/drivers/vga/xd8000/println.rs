///----------------------------------------///
///*****************VGA********************///
/// VGA println implementation for XD8000  ///
///****************************************///
///----------------------------------------///
// Using Color from the color module
use crate::drivers::vga::xd8000::color::Color;

use spin::Mutex;

// Using core::fmt::Write for implementing the Write trait
use core::{fmt::Write, iter};

// CODE IMPLEMENTATION
// Static mutable buffers for text and color attributes
struct VgaState {
    skreen_text: [u8; 25 * 80],
    skreen_color: [u8; 25 * 80],
    row: usize,
    col: usize,
    write_color: u8,
}

static STATE: Mutex<VgaState> = Mutex::new(VgaState::new());

fn vga_print(args: core::fmt::Arguments, color: u8) {
    let mut printer = VGAPrinter;

    STATE.lock().write_color = color;
    core::fmt::Write::write_fmt(&mut printer, args).unwrap();
}

//*-END MACROS println!();-*/
pub fn println(args: core::fmt::Arguments) {
    vga_print(args, Color::WHITE);
}

//*-END MACROS println_warn();-*/
pub fn println_warn(args: core::fmt::Arguments) {
    vga_print(args, Color::warning());
}

//*-END MACROS println_warn();-*/
pub fn println_error(args: core::fmt::Arguments) {
    vga_print(args, Color::error());
}

struct VGAPrinter;

// Implementing the Write trait for VGAPrinter
impl Write for VGAPrinter {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        //let text = text.bytes().enumerate();
        //call the buffer function
        let mut state = STATE.lock();

        for byte in text.bytes() {
            match byte {
                b'\n' => state.next_line(),
                b'\r' => state.first_line(),
                b'\t' => state.tab(),
                32..=126 => state.write_char(byte),
                _ => {}
            }
        }

        Ok(())
    }
}

impl VgaState {
    const fn new() -> Self {
        Self {
            skreen_text: [0; 25 * 80],
            skreen_color: [0; 25 * 80],
            row: 0,
            col: 0,
            write_color: Color::WHITE,
        }
    }
    fn write_char(&mut self, byte: u8) {
        let idx = self.row * 80 + self.col;

        self.skreen_text[idx] = byte;
        self.skreen_color[idx] = self.write_color;

        let base_addr = 0xb8000 as *mut u8;
        unsafe {
            let addr = base_addr.add(idx * 2);
            *addr = self.skreen_text[idx];
            *addr.offset(1) = self.skreen_color[idx];
        }
        self.col += 1;
    }

    fn first_line(&mut self) {}

    fn next_line(&mut self) {}

    fn tab(&mut self) {}
}
