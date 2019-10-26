use std::fs::File;
use std::fmt::{self, Display, Formatter};

pub const PALETTE: [Colour; 16] = [
	Colour(0x00,0x00,0x00),  // 0 black
	Colour(0x00,0x00,0xAA),  // 1 blue
	Colour(0x00,0xAA,0x00),  // 2 green
	Colour(0x00,0xAA,0xAA),  // 3 cyan
	Colour(0xAA,0x00,0x00),  // 4 red
	Colour(0xAA,0x00,0xAA),  // 5 magenta
	Colour(0xAA,0x55,0x00),  // 6 brown
	Colour(0xAA,0xAA,0xAA),  // 7 light gray
	Colour(0x55,0x55,0x55),  // 8 dark gray
	Colour(0x55,0x55,0xFF),  // 9 light blue
	Colour(0x55,0xFF,0x55),  // 10 light green
	Colour(0x55,0xFF,0xFF),  // 11 light cyan
	Colour(0xFF,0x55,0x55),  // 12 light red
	Colour(0xFF,0x55,0xFF),  // 13 light magenta
	Colour(0xFF,0xFF,0x55),  // 14 yellow
	Colour(0xFF,0xFF,0xFF)   // 15 white
];
pub const MODES: [Mode; 4] = [
	Mode( 0, 2, 4, 6), // P0Lo: black, green, red, brown 
	Mode( 0,10,12,14), // P0Hi: black, lt.green, lt.red, yellow
	Mode( 0, 3, 5, 7), // P1Lo: black, cyan, magenta, lt.gray
	Mode( 0,11,13,15)  // P1Hi: black, lt.cyan, lt.magenta, white
];
pub enum GraphicsMode {
	P0LO, P0HI, P1LO, P1HI
}

#[derive(Eq,PartialEq,PartialOrd)]
pub struct Colour(u8,u8,u8);
impl Display for Colour {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

pub struct Mode(u8,u8,u8,u8);

pub struct Image {
	pub width: u32,
	pub height: u32,
	pub rgba_data: Vec<u8>,
	pub p_data: Vec<u8>
}
impl Image {
	fn to_indexed(data: &Vec<u8>) -> Vec<u8> {
		if data.len() % 4 != 0 {
			panic!("Data size not divisible by 4 (not RGBA image???)");
		}
		let l = data.len()/4;
		let mut converted = vec![0 as u8; l];
		for n in 0..l {
			let dp = n * 4;
			let r = data[dp];
			let g = data[dp+1];
			let b = data[dp+2];
			// skip "let a = data[dp+3]" because we don't need the alpha value
			let p = Colour(r,g,b);

			let mut cidx: u8 = 16;
			for colour in 0..=15 {
				let c = &PALETTE[colour];
				if c == &p {
					cidx = colour as u8;
					break;
				}
			}
			if cidx == 16 {
				panic!("Pixel {} offset {}, colour not in palette: {}", n, dp, p);
			}
			converted[n] = cidx;
		}
		return converted;
	}
	pub fn from_png(input_file_name: String) -> Image {
		let decoder = png::Decoder::new(File::open(input_file_name).unwrap());
		let (info, mut reader) = decoder.read_info().unwrap();

		println!("Input image size: {}x{}",info.width,info.height);

		// Allocate the buffer
		let mut buf = vec![0; info.buffer_size()];

		// Decode image
		reader.next_frame(&mut buf).unwrap();

		// Make the indexed palette
		let p_data = Image::to_indexed(&buf);
		
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
		return result;
	}
	pub fn match_palette() {

	}
	pub fn dump_bitmap(&self) {
		for y in 0..self.height {
			for x in 0..self.width {
				let i = (y*self.width+x) as usize;
				let ch = self.p_data[i];
				if ch < 8 {
					print!("\x1b[{}m",40+ch);
				} else {
					print!("\x1b[{}m",100+ch-8);
				}
				print!("{:02X}",ch);
			}
			print!("\x1b[m");
			println!("");
		}
	}
}

