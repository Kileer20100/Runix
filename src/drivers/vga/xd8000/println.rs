///----------------------------------------///
///*****************VGA********************///
/// VGA println implementation for XD8000  ///
///****************************************///
///----------------------------------------///
// Import color definitions
use crate::drivers::vga::xd8000::color::Color;

// Spin-based mutex for safe global state in no_std environment
use spin::Mutex;

// Required for implementing the Write trait
use core::fmt::Write;

/// VGA text buffer state.
/// Manages an in-memory copy of the screen (25x80 characters),
/// current cursor position, and active text color.
struct VgaState {
    /// Character buffer (25 rows × 80 columns)
    screen_text: [u8; 25 * 80],
    /// Color attribute buffer (parallel to screen_text)
    screen_color: [u8; 25 * 80],
    /// Current cursor row (0–24)
    row: usize,
    /// Current cursor column (0–79)
    col: usize,
    /// Active text color for new output
    write_color: u8,
}

/// Global VGA state, protected by a spinlock for thread safety.
static STATE: Mutex<VgaState> = Mutex::new(VgaState::new());

/// Internal helper: sets the output color and formats the message.
fn vga_print(args: core::fmt::Arguments, color: u8) {
    let mut printer = VGAPrinter;
    // Set the color before formatting
    STATE.lock().write_color = color;
    // Format and write the message
    core::fmt::Write::write_fmt(&mut printer, args).unwrap();
}

/// Prints a formatted string with default (white) color and appends a newline.
pub fn println(args: core::fmt::Arguments) {
    vga_print(args, Color::WHITE);
}

/// Prints a warning message in yellow.
pub fn println_warn(args: core::fmt::Arguments) {
    vga_print(args, Color::warning());
}

/// Prints an error message in red.
pub fn println_error(args: core::fmt::Arguments) {
    vga_print(args, Color::error());
}

/// Writer implementation that routes formatted output to the VGA buffer.
struct VGAPrinter;

impl Write for VGAPrinter {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        let mut state = STATE.lock();

        // Process each byte of the input string
        for byte in text.bytes() {
            match byte {
                b'\n' => state.next_line(),              // Line feed
                b'\r' => state.first_line(),             // Carriage return (reset to top-left)
                b'\t' => state.tab(),                    // Tab (simple 4-space jump)
                32..=126 => state.main_write_char(byte), // Printable ASCII
                _ => {}                                  // Ignore non-printable characters
            }
        }

        Ok(())
    }
}

impl VgaState {
    /// Creates a new, cleared VGA state.
    const fn new() -> Self {
        Self {
            screen_text: [0; 25 * 80],
            screen_color: [0; 25 * 80],
            row: 0,
            col: 0,
            write_color: Color::WHITE,
        }
    }

    /// Writes a single printable character to the buffer and screen.
    fn main_write_char(&mut self, byte: u8) {
        self.render_vga_char(byte); // Write to buffer and VGA memory
        self.check_buffer(); // Ensure we're within bounds
    }

    /// Renders a single character to both the internal buffer and VGA hardware memory.
    fn render_vga_char(&mut self, byte: u8) {
        let idx = self.row * 80 + self.col;
        let base_addr = 0xb8000 as *mut u8;

        // Update internal buffers
        self.screen_text[idx] = byte;
        self.screen_color[idx] = self.write_color;

        // Write to VGA memory (character + color)
        unsafe {
            let addr = base_addr.add(idx * 2);
            *addr = self.screen_text[idx];
            *addr.offset(1) = self.screen_color[idx];
        }
        self.col += 1; // Advance cursor
    }

    /// Re-renders the entire screen from the internal buffer.
    /// Used after bulk operations like scrolling or clearing.
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

    /// Checks if cursor is at the end of line or bottom of screen.
    /// Triggers line wrap or scrolling if needed.
    fn check_buffer(&mut self) {
        if self.col >= 80 {
            self.next_line(); // Wrap to next line
        }
        if self.row == 24 && self.col >= 80 {
            self.scroll_up(); // Scroll when at bottom
        }
    }

    /// Scrolls the entire screen up by one line.
    /// The top line is discarded, and the bottom line is cleared.
    fn scroll_up(&mut self) {
        // Shift all lines up: line 1 → line 0, line 2 → line 1, ..., line 24 → line 23
        for row in 1..25 {
            for col in 0..80 {
                let src_idx = row * 80 + col;
                let dst_idx = (row - 1) * 80 + col;
                self.screen_text[dst_idx] = self.screen_text[src_idx];
                self.screen_color[dst_idx] = self.screen_color[src_idx];
            }
        }

        // Clear the last line (now empty after scroll)
        let last_row = 24 * 80;
        for i in 0..80 {
            self.screen_text[last_row + i] = b' ';
            self.screen_color[last_row + i] = Color::BLACK;
        }

        // Refresh the entire screen
        self.full_render_buffer();
        // Keep cursor at the beginning of the last line
        self.row = 24;
        self.col = 0;
    }

    /// Clears the entire screen and resets the cursor to (0, 0).
    pub fn clear_buffer(&mut self) {
        self.screen_text = [0; 25 * 80];
        self.screen_color = [Color::BLACK; 25 * 80];
        self.col = 0;
        self.row = 0;
        self.full_render_buffer();
    }

    /// Clears the first line and moves cursor to top-left.
    /// Note: This is a simple reset; not standard terminal behavior.
    fn first_line(&mut self) {
        for i in 0..80 {
            self.screen_text[i] = b' ';
            self.screen_color[i] = Color::BLACK;
        }
        self.row = 0;
        self.col = 0;
        self.full_render_buffer();
    }

    /// Moves cursor to the beginning of the next line.
    /// If already on the last line, scrolling must be handled externally.
    fn next_line(&mut self) {
        if self.row < 24 {
            self.row += 1;
            self.col = 0;
        } else {
            self.scroll_up();
        }
        // If on last line (row == 24), scroll_up() is called vi a check_buffer()
    }

    /// Simple tab implementation: advance cursor by 4 columns.
    /// Does not handle line wrapping.
    fn tab(&mut self) {
        self.col += 4;
    }
}
