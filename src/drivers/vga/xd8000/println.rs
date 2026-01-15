///----------------------------------------///
///*****************VGA********************///
/// VGA println implementation for XD8000  ///
///****************************************///
///----------------------------------------///
// Using Color from the color module
use crate::drivers::vga::xd8000::color::Color;

use spin::Mutex;

// Using core::fmt::Write for implementing the Write trait
use core::fmt::Write;

// CODE IMPLEMENTATION
// Static mutable buffers for text and color attributes
struct VgaState {
    screen_text: [u8; 25 * 80],
    screen_color: [u8; 25 * 80],
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
        //coll the buffer function
        let mut state = STATE.lock();

        for byte in text.bytes() {
            match byte {
                b'\n' => state.next_line(),
                b'\r' => state.first_line(),
                b'\t' => state.tab(),
                32..=126 => state.main_write_char(byte),
                _ => {}
            }
        }

        Ok(())
    }
}

impl VgaState {
    const fn new() -> Self {
        Self {
            screen_text: [0; 25 * 80],
            screen_color: [0; 25 * 80],
            row: 0,
            col: 0,
            write_color: Color::WHITE,
        }
    }

    fn main_write_char(&mut self, byte: u8) {
        self.check_buffer();
        self.render_vga_char(byte);
        self.col += 1;
    }

    fn render_vga_char(&mut self, byte: u8) {
        let idx = self.row * 80 + self.col;
        let base_addr = 0xb8000 as *mut u8;

        self.screen_text[idx] = byte;
        self.screen_color[idx] = self.write_color;

        unsafe {
            let addr = base_addr.add(idx * 2);
            *addr = self.screen_text[idx];
            *addr.offset(1) = self.screen_color[idx];
        }
    }
    fn full_render_buffer(&mut self) {
        let base_addr = 0xb8000 as *mut u8;
        unsafe {
            for idx in 0..self.screen_text.len() {
                let addr = base_addr.add(idx * 2);
                *addr = self.screen_text[idx];
                *addr.offset(1) = self.screen_color[idx];
            }
        }
    }
    fn check_buffer(&mut self) {
        if self.col >= 80 {
            self.next_line();
        }
        if self.row == 24 {
            self.scroll_up();
        }
    }

    fn scroll_up(&mut self) {
        for row in 1..25 {
            for col in 0..80 {
                let row_idx = row * 80 + col;
                let dst_idx = (row - 1) * 80 + col;
                self.screen_text[dst_idx] = self.screen_text[row_idx];
                self.screen_color[dst_idx] = self.screen_color[row_idx];
            }
        }

        let last_row = 24 * 80;

        for i in 0..80 {
            self.screen_text[last_row + i] = b' ';
            self.screen_color[last_row + i] = Color::BLACK;
        }

        self.full_render_buffer();
        self.row = 24;
        self.col = 0;
    }

    pub fn clear_buffer(&mut self) {
        self.screen_text = [0; 25 * 80];
        self.screen_color = [Color::BLACK; 25 * 80];
        self.col = 0;
        self.row = 0;
        self.full_render_buffer();
    }
    fn first_line(&mut self) {
        for i in 0..80 {
            self.screen_text[i] = b' ';
            self.screen_color[i] = Color::BLACK;
        }
        self.row = 0;
        self.col = 0;
        self.full_render_buffer();
    }

    fn next_line(&mut self) {
        if self.row < 24 {
            self.row += 1;
            self.col = 0;
        }
    }

    fn tab(&mut self) {
        self.col += 4;
    }
}
