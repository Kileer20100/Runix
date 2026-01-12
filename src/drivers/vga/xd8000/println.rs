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
static SKREEN_TEXT: Mutex<[u8; 25 * 80]> = Mutex::new([0; 25 * 80]);
static SKREEN_COLOR: Mutex<[u8; 25 * 80]> = Mutex::new([0; 25 * 80]);

struct VgaState {
    iter: u32,
    row_write: u32,
    write_color: u8,
}

impl VgaState {
    const fn new() -> Self {
        Self {
            iter: 0,
            row_write: 0,
            write_color: Color::WHITE,
        }
    }
}

static STATE: Mutex<VgaState> = Mutex::new(VgaState::new());

//*-END MACROS println!();-*/
pub fn println(args: core::fmt::Arguments) {
    let mut printer = VGAPrinter;

    STATE.lock().write_color = Color::WHITE;

    core::fmt::Write::write_fmt(&mut printer, args).unwrap();
}

//*-END MACROS println_warn();-*/
pub fn println_warn(args: core::fmt::Arguments) {
    let mut printer = VGAPrinter;

    STATE.lock().write_color = Color::warning();

    core::fmt::Write::write_fmt(&mut printer, args).unwrap();
}

//*-END MACROS println_warn();-*/
pub fn println_error(args: core::fmt::Arguments) {
    let mut printer = VGAPrinter;

    STATE.lock().write_color = Color::error();

    core::fmt::Write::write_fmt(&mut printer, args).unwrap();
}

struct VGAPrinter;

// Implementing the Write trait for VGAPrinter
impl Write for VGAPrinter {
    fn write_str(&mut self, message: &str) -> core::fmt::Result {
        //let text = message.bytes().enumerate();
        //call the buffer function
        bufer_vga(message);

        //write the buffered data to VGA memory
        let base = 0xb8000 as *mut u8;

        //- - - - - - - - - - - - - - - - - - - - - - - - -//
        //get the global iteration and calculate the row
        let global_iteration = STATE.lock().iter;
        let row = global_iteration / 80;
        let vga_buffer = unsafe {
            if global_iteration == 0 {
                0xb8000 as *mut u8
            } else {
                base.add((row as usize) * 0xa0)
            }
        };
        //- - - - - - - - - - - - - - - - - - - - - - - - -//
        //write the current row from the buffers to VGA memory
        //- - - - - - - - - - - - - - - - - - - - - - - - -//
        //get the slice for the current row
        unsafe {
            //get the slice indices for the current row
            let (start, stop) = slise_buffer_row();
            //get the slices from the text and color buffers
            let slise_text_buffer = &SKREEN_TEXT.lock()[start..stop];
            let slise_color_buffer = &SKREEN_COLOR.lock()[start..stop];
            //write the slices to VGA memory

            for i in 0..80 {
                //write character byte
                *vga_buffer.offset(i as isize * 2) = slise_text_buffer[i];
                //write color byte
                *vga_buffer.offset(i as isize * 2 + 1) = slise_color_buffer[i];
            }
            Ok(())
        }
        //- - - - - - - - - - - - - - - - - - - - - - - - -//
    }
}

//*-Function to buffer VGA output-*/
fn bufer_vga(message: &str) {
    //convert message to bytes
    let text = message.as_bytes();

    //get the global write color
    let global_color = STATE.lock().write_color;

    //call global iteration function
    global_iteration_();

    //get the start base for writing
    let start_base = (STATE.lock().row_write as usize) * 80;

    //write the text and color to the buffers
    unsafe {
        for (i, &byte) in text.iter().enumerate() {
            let idx = start_base + i;
            if idx >= 25 * 80 {
                break;
            }

            SKREEN_TEXT.lock()[idx] = byte;
            SKREEN_COLOR.lock()[idx] = global_color as u8;

            STATE.lock().iter = (idx + 1) as u32;
        }
    }
}
//*-Function to manage global iteration and row writing-*/
fn global_iteration_() {
    //get the global iteration
    let global_iteration = STATE.lock().iter;

    //calculate the current row
    let row = global_iteration / 80;
    //update the ROW_WRITE based on the current row
    unsafe {
        let row_write: u32;

        if row == 0 {
            //check if the first two characters are non-zero
            let logick_slise = &SKREEN_TEXT.lock()[0..2];

            if logick_slise[0] != 0 && logick_slise[1] != 0 {
                //move to the next row
                row_write = 1;
            } else {
                //reset to the first row
                row_write = 0;
            }
        } else if row > 0 && row < 24 {
            //move to the next row
            row_write = row + 1;
        } else {
            //clear the buffer and reset to the first row
            clear_buffer();
            row_write = 0;
        }
        //update the ROW_WRITE variable
        STATE.lock().row_write = row_write;
    }
}
//*-Function to clear the text and color buffers-*/
fn clear_buffer() {
    unsafe {
        //clear the SKREEN_TEXT and SKREEN_COLOR buffers
        for i in 0..(25 * 80) {
            SKREEN_TEXT.lock()[i] = 0;
            SKREEN_COLOR.lock()[i] = 0;
        }

        //clear the VGA memory
        let base = 0xb8000 as *mut u8;
        //set each character to space and color to black
        for i in 0..(25 * 80) {
            *base.add(i * 2) = b' ';
            *base.add(i * 2 + 1) = Color::BLACK;
        }
    }
}
//*-Function to get the slice indices for the current row in the buffers-*/
fn slise_buffer_row() -> (usize, usize) {
    //get the global iteration
    let global_iteration = STATE.lock().iter;
    //calculate the current row
    let row = global_iteration / 80;
    //return the start and stop indices for the current row
    ((80 * row) as usize, (80 * (row + 1)) as usize)
}
