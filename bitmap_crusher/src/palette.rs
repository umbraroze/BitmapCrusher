
use std::fmt::{self, Display, Formatter};

pub const PALETTE: [PaletteEntry; 16] = [
	PaletteEntry { rgb_colour: Colour(0x00, 0x00, 0x00), ansi_bg: 40 },  // 0 black
	PaletteEntry { rgb_colour: Colour(0x00, 0x00, 0xAA), ansi_bg: 44 },  // 1 blue
	PaletteEntry { rgb_colour: Colour(0x00, 0xAA, 0x00), ansi_bg: 42 },  // 2 green
	PaletteEntry { rgb_colour: Colour(0x00, 0xAA, 0xAA), ansi_bg: 46 },  // 3 cyan
	PaletteEntry { rgb_colour: Colour(0xAA, 0x00, 0x00), ansi_bg: 41 },  // 4 red
	PaletteEntry { rgb_colour: Colour(0xAA, 0x00, 0xAA), ansi_bg: 45 },  // 5 magenta
	PaletteEntry { rgb_colour: Colour(0xAA, 0x55, 0x00), ansi_bg: 43 },  // 6 brown
	PaletteEntry { rgb_colour: Colour(0xAA, 0xAA, 0xAA), ansi_bg: 47 },  // 7 light gray
	PaletteEntry { rgb_colour: Colour(0x55, 0x55, 0x55), ansi_bg: 100 }, // 8 dark gray
	PaletteEntry { rgb_colour: Colour(0x55, 0x55, 0xFF), ansi_bg: 104 }, // 9 light blue
	PaletteEntry { rgb_colour: Colour(0x55, 0xFF, 0x55), ansi_bg: 102 }, // 10 light green
	PaletteEntry { rgb_colour: Colour(0x55, 0xFF, 0xFF), ansi_bg: 106 }, // 11 light cyan
	PaletteEntry { rgb_colour: Colour(0xFF, 0x55, 0x55), ansi_bg: 101 }, // 12 light red
	PaletteEntry { rgb_colour: Colour(0xFF, 0x55, 0xFF), ansi_bg: 105 }, // 13 light magenta
	PaletteEntry { rgb_colour: Colour(0xFF, 0xFF, 0x55), ansi_bg: 103 }, // 14 yellow
	PaletteEntry { rgb_colour: Colour(0xFF, 0xFF, 0xFF), ansi_bg: 107 }  // 15 white
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
pub struct Colour(u8,u8,u8);
impl Display for Colour {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}",self.0,self.1,self.2)
    }
}

#[derive(Eq,PartialEq,PartialOrd)]
pub struct PaletteEntry {
    rgb_colour:Colour,
    ansi_bg:u8
}
impl Display for PaletteEntry {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (ANSI: {})",self.rgb_colour,self.ansi_bg)
    }
}

#[derive(Eq,PartialEq,PartialOrd)]
pub struct Mode {
    name: &'static str,
    colours: [&'static PaletteEntry;4],
}
impl Display for Mode {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, Colours: {} {} {} {}]",
            self.name,
            self.colours[0], self.colours[1], self.colours[2], self.colours[3])
    }
}
