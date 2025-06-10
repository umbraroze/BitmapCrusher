extern crate png;
//extern crate hexdump;

use std::fs::File;
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
		colour_indexes: [0,2,4,6],
        //        black,       green,        red,          brown 
        colours: [&PALETTE[0], &PALETTE[2],  &PALETTE[4],  &PALETTE[6]]
    },
	Mode {
        name: "Palette 0 High",
		colour_indexes: [0,10,12,14],
        //        black,       lt.green,     lt.red,       yellow
        colours: [&PALETTE[0], &PALETTE[10], &PALETTE[12], &PALETTE[14]]
    }, 
	Mode {
        name: "Palette 1 Low", 
		colour_indexes: [0,3,5,7],
        //        black,       cyan,         magenta,      lt.gray
        colours: [&PALETTE[0], &PALETTE[3],  &PALETTE[5],  &PALETTE[7]]
    },
	Mode {
        name: "Palette 1 High",
		colour_indexes: [0,11,13,15],
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
	colour_indexes: [u8;4],
    colours: [&'static PaletteEntry;4],
}
impl Display for Mode {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, Colours: {},{},{},{} - {}, {}, {}, {}]",
            self.name,
			self.colour_indexes[0],self.colour_indexes[1],
			self.colour_indexes[2],self.colour_indexes[3],
            self.colours[0], self.colours[1],
			self.colours[2], self.colours[3])
    }
}

pub struct Image {
	pub width: u32,
	pub height: u32,
	pub rgba_data: Vec<u8>,
	pub p_data: Vec<u8>
}
impl Image {
	fn to_indexed(data: &Vec<u8>, bpp: usize) -> Vec<u8> {
		if data.len() % bpp != 0 {
			panic!("Data size not divisible by bytes per pixel");
		}
		let l = data.len()/bpp;
		let mut converted = vec![0 as u8; l];
		for n in 0..l {
			let dp = n * bpp;
			let r = data[dp];
			let g = data[dp+1];
			let b = data[dp+2];
			// skip "let a = data[dp+3]" because we don't need the alpha value
			// and that would be an off-by-one anyway if bpp == 3
			let p = Colour(r,g,b);

			let mut cidx: u8 = 16;
			for colour in 0..=15 {
				let c = &PALETTE[colour];
				if &c.rgb_colour == &p {
					cidx = colour as u8;
					break;
				}
			}
			if cidx == 16 {
				//hexdump::hexdump(data);
				panic!("Pixel {} offset {} ({:x}), {} not in palette",
					n, dp, dp, p);
			}
			converted[n] = cidx;
		}
		return converted;
	}
	pub fn from_png(input_file_name: String) -> Image {
		let decoder = png::Decoder::new(File::open(input_file_name).unwrap());
		let mut reader = decoder.read_info().unwrap();
		// Allocate the buffer
		let mut buf = vec![0; reader.output_buffer_size()];
		// Decode image
		let info = reader.next_frame(&mut buf).unwrap();

		assert_eq!(info.bit_depth,png::BitDepth::Eight,
			"Only 8-bit images supported.");
		// Determine how many bytes per pixel
		let bpp: usize = match info.color_type {
			png::ColorType::Rgb => 3,
			png::ColorType::Rgba => 4,
			_ => panic!("Only RGB and RGBA images supported")
		};
		println!("Colour type: {:?}, {} bytes per pixel", info.color_type, bpp);

		println!("Input image size: {}x{}",info.width,info.height);

		let expected_len: usize =
			info.width as usize * info.height as usize * bpp;
		assert_eq!(buf.len(),expected_len,
			"{} bytes of data decoded, expected {}",
			buf.len(),
			expected_len);

		// Make the indexed palette
		let p_data = Image::to_indexed(&buf,bpp);
		
		Image {
			width: info.width,
			height: info.height,
			rgba_data: buf,
			p_data: p_data
		}
	}
	pub fn find_colours(&self) -> Vec<u8> {
		let mut result: Vec<u8> = vec![];
		for i in 0..self.p_data.len() {
			let mut found = false;
			for c in 0..result.len() {
				if self.p_data[i] == result[c] {
					found = true; break;
				}
			}
			if !found {
				result.push(self.p_data[i]);
			}
		}
		result.sort();
		return result;
	}
	pub fn match_palette(colours: Vec<u8>) -> Option<usize> {
		assert_eq!(colours.len(),4,
			"Palette size not 4, can't convert to a mode");

		// FIXME: This should be neater
		for i in 0..MODES.len() {
			// Loop through colour indexes; if any of them is different,
			// quit trying.
			let mut found_difference = false;
			'colors: for c in 0..3 as usize {
				if !found_difference && MODES[i].colour_indexes[c] != colours[c] {
					// We found a difference!
					found_difference = true;
					break 'colors;
				}
			}
			// If we 
			if !found_difference {
				return Some(i)
			}
		};
		return None;
	}
	pub fn dump_bitmap(&self, ansi_colours: bool) {
		for y in 0..self.height {
			for x in 0..self.width {
				let i = (y*self.width+x) as usize;
				let ch = self.p_data[i];
				if ansi_colours {
					print!("\x1b[{}m",PALETTE[ch as usize].ansi_bg);
				}
				print!("{:02X}",ch);
			}
			if ansi_colours {
				print!("\x1b[m");
			}
			println!("");
		}
	}
}

