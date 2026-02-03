pub struct Color;

impl Color {
    pub const BLACK:         u8 = 0x0;
    pub const BLUE:          u8 = 0x1;
    pub const GREEN:         u8 = 0x2;
    pub const CYAN:          u8 = 0x3;
    pub const RED:           u8 = 0x4;
    pub const MAGENTA:       u8 = 0x5;
    pub const BROWN:         u8 = 0x6;
    pub const LIGHT_GRAY:    u8 = 0x7;
    pub const DARK_GRAY:     u8 = 0x8;
    pub const LIGHT_BLUE:    u8 = 0x9;
    pub const LIGHT_GREEN:   u8 = 0xa;
    pub const LIGHT_CYAN:    u8 = 0xb;
    pub const LIGHT_RED:     u8 = 0xc;
    pub const LIGHT_MAGENTA: u8 = 0xd;
    pub const YELLOW:        u8 = 0xe;
    pub const WHITE:         u8 = 0xf;

    pub fn make_color(foreground: u8, background: u8) -> u8 {
        (background << 4) | (foreground & 0x0F)
    }
    
    pub fn error() -> u8 {
        Self::make_color(Self::RED, Self::BLACK)
    }
    pub fn warning() -> u8 {
        Self::make_color(Self::YELLOW, Self::BLACK)
    }
    pub fn text_write() -> u8 {
        Self::WHITE
    }


} 