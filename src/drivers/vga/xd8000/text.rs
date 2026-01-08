
use crate::drivers::vga::xd8000::color::{Color};

pub fn text_write(){
    let vga_buffer = 0xb8000 as *mut u8;

    let text = b"Hello! Hello! Hello! Hello!";

    let mut iteration:u8;
    let mut color_set:u8 = Color::text_write();
    for (i, &byte) in text.iter().enumerate(){

        iteration = i as u8;

        match iteration{
            0 => color_set = Color::text_write(),
            7 => color_set = Color::error(),
            13 => color_set = Color::warning(),
            20 => color_set = Color::make_color(Color::BLUE, Color::GREEN),
            _ => (),

        }
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = color_set;
        }
        }
}