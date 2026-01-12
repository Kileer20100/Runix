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
    skreen_text: [u8; 25 * 80],
    skreen_color: [u8; 25 * 80],
    iter: u32,
    row_write: u32,
    write_color: u8,
}

impl VgaState {
    const fn new() -> Self {
        Self {
            skreen_text: [0; 25 * 80],
            skreen_color: [0; 25 * 80],
            iter: 0,
            row_write: 0,
            write_color: Color::WHITE,
        }
    }
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
    fn write_str(&mut self, message: &str) -> core::fmt::Result {
        //let text = message.bytes().enumerate();
        //call the buffer function
        let mut state = STATE.lock();

        bufer_vga(message, &mut state);

        //write the buffered data to VGA memory
        let base = 0xb8000 as *mut u8;

        //- - - - - - - - - - - - - - - - - - - - - - - - -//
        //get the global iteration and calculate the row
        let global_iteration = state.iter;
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
            let (start, stop) = slise_buffer_row(&mut state);
            //get the slices from the text and color buffers
            let slise_text_buffer = &state.skreen_text[start..stop];
            let slise_color_buffer = &state.skreen_color[start..stop];
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
fn bufer_vga(message: &str, state: &mut VgaState) {
    //convert message to bytes
    let text = message.as_bytes();

    //get the global write color
    let global_color = state.write_color;

    //call global iteration function
    global_iteration_(state);

    //get the start base for writing
    let start_base = (state.row_write as usize) * 80;

    //write the text and color to the buffers
    {
        for (i, &byte) in text.iter().enumerate() {
            let idx = start_base + i;
            if idx >= 25 * 80 {
                break;
            }

            state.skreen_text[idx] = byte;
            state.skreen_color[idx] = global_color as u8;

            state.iter = (idx + 1) as u32;
        }
    }
}
//*-Function to manage global iteration and row writing-*/
fn global_iteration_(state: &mut VgaState) {
    //get the global iteration
    let global_iteration = state.iter;

    //calculate the current row
    let row = global_iteration / 80;
    //update the ROW_WRITE based on the current row
    {
        let row_write: u32;

        if row == 0 {
            //check if the first two characters are non-zero
            let logick_slise = &state.skreen_text[0..2];

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
            clear_buffer(state);
            row_write = 0;
        }
        //update the ROW_WRITE variable
        state.row_write = row_write;
    }
}
//*-Function to clear the text and color buffers-*/
fn clear_buffer(state: &mut VgaState) {
    unsafe {
        //clear the SKREEN_TEXT and SKREEN_COLOR buffers
        for i in 0..(25 * 80) {
            state.skreen_text[i] = 0;
            state.skreen_color[i] = 0;
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
fn slise_buffer_row(state: &mut VgaState) -> (usize, usize) {
    //get the global iteration
    let global_iteration = state.iter;
    //calculate the current row
    let row = global_iteration / 80;
    //return the start and stop indices for the current row
    ((80 * row) as usize, (80 * (row + 1)) as usize)
}
