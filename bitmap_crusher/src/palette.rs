
use std::fmt::{self, Display, Formatter};

pub const PALETTE: [Colour; 16] = [
	Colour { r: 0x00, g: 0x00, b: 0x00, ansi_bg: 40 },   // 0 black
	Colour { r: 0x00, g: 0x00, b: 0xAA, ansi_bg: 44 },   // 1 blue
	Colour { r: 0x00, g: 0xAA, b: 0x00, ansi_bg: 42 },   // 2 green
	Colour { r: 0x00, g: 0xAA, b: 0xAA, ansi_bg: 46 },   // 3 cyan
	Colour { r: 0xAA, g: 0x00, b: 0x00, ansi_bg: 41 },   // 4 red
	Colour { r: 0xAA, g: 0x00, b: 0xAA, ansi_bg: 45 },   // 5 magenta
	Colour { r: 0xAA, g: 0x55, b: 0x00, ansi_bg: 43 },   // 6 brown
	Colour { r: 0xAA, g: 0xAA, b: 0xAA, ansi_bg: 47 },   // 7 light gray
	Colour { r: 0x55, g: 0x55, b: 0x55, ansi_bg: 100 },  // 8 dark gray
	Colour { r: 0x55, g: 0x55, b: 0xFF, ansi_bg: 104 },  // 9 light blue
	Colour { r: 0x55, g: 0xFF, b: 0x55, ansi_bg: 102 },  // 10 light green
	Colour { r: 0x55, g: 0xFF, b: 0xFF, ansi_bg: 106 },  // 11 light cyan
	Colour { r: 0xFF, g: 0x55, b: 0x55, ansi_bg: 101 },  // 12 light red
	Colour { r: 0xFF, g: 0x55, b: 0xFF, ansi_bg: 105 },  // 13 light magenta
	Colour { r: 0xFF, g: 0xFF, b: 0x55, ansi_bg: 103 },  // 14 yellow
	Colour { r: 0xFF, g: 0xFF, b: 0xFF, ansi_bg: 107 }   // 15 white
];
pub const MODES: [Mode; 4] = [
	Mode {
        name: "Palette 0 Low",
        //        black,       green,        red,          brown 
        colours: [&PALETTE[0], &PALETTE[2],  &PALETTE[4],  &PALETTE[6]]
    },
	Mode {
        name: "Palette 0 High",
        //        black,       lt.green,     lt.red,       yellow
        colours: [&PALETTE[0], &PALETTE[10], &PALETTE[12], &PALETTE[14]]
    }, 
	Mode {
        
        name: "Palette 1 Low", 
        //        black,       cyan,         magenta,      lt.gray
        colours: [&PALETTE[0], &PALETTE[3],  &PALETTE[5],  &PALETTE[7]]
    },
	Mode {
        name: "Palette 1 High",
        //        black,       lt.cyan,      lt.magenta,   white
        colours: [&PALETTE[0], &PALETTE[11], &PALETTE[13], &PALETTE[15]]
    }
];

#[derive(Eq,PartialEq,PartialOrd)]
pub struct Colour {
    r:u8,
    g:u8,
    b:u8,
    ansi_bg:u8
}
impl Colour {
    fn match_rgb(&self, r:u8, g:u8, b:u8) -> bool {
        return self.r == r && self.g == g && self.b == b;
    }
}
impl Display for Colour {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[derive(Eq,PartialEq,PartialOrd)]
pub struct Mode {
    name: &'static str,
    colours: [&'static Colour;4],
}
impl Display for Mode {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, Colours: {} {} {} {}]",
            self.name,
            self.colours[0], self.colours[1], self.colours[2], self.colours[3])
    }
}
