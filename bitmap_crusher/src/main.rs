
use std::fs::File;

// TODO: Make the struct fancily comparable with just an ==
// TODO: Some kind of a function to convert the colour to a string.
struct Colour(u8,u8,u8);
struct Mode(u8,u8,u8,u8);

fn compare_colour(a:&Colour,b:&Colour) -> bool {
	return a.0 == b.0 && a.1 == b.1 && a.2 == b.2;
}

fn load_png(input_file_name: String) -> (u32,u32,Vec<u8>) {
	let decoder = png::Decoder::new(File::open(input_file_name).unwrap());
	let (info, mut reader) = decoder.read_info().unwrap();

	println!("Input image size: {}x{}",info.width,info.height);

	// Allocate the buffer
	let mut buf = vec![0; info.buffer_size()];

	// Decode image
	reader.next_frame(&mut buf).unwrap();

	return (info.width,info.height,buf);
}

const PALETTE: [Colour; 16] = [
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
const MODES: [Mode; 4] = [
	Mode( 0, 2, 4, 6), // P0Lo: black, green, red, brown 
	Mode( 0,10,12,14), // P0Hi: black, lt.green, lt.red, yellow
	Mode( 0, 3, 5, 7), // P1Lo: black, cyan, magenta, lt.gray
	Mode( 0,11,13,15)  // P1Hi: black, lt.cyan, lt.magenta, white
];

fn to_indexed(data: &Vec<u8>) -> Vec<u8> {
	assert_eq!(data.len() % 4, 0,
		"Data size not divisible by 4 (not RGBA image???)");
	let l = data.len()/4;
	let mut converted = vec![0 as u8; l];
	for n in 0..l {
		let dp = n * 4;
		let ra = data[dp];
		let ga = data[dp+1];
		let ba = data[dp+2];
		let p = Colour(ra,ga,ba);

		let mut cidx: u8 = 16;
		for colour in 0..=15 {
			let c = &PALETTE[colour];
			if compare_colour(c,&p) {
				cidx = colour as u8;
				break;
			}
		}
		assert_ne!(cidx,16,
			"Pixel {} offset {}, colour not in palette: #{:02X}{:02X}{:02X}",
			n, dp, p.0,p.1,p.2);
		converted[n] = cidx;
	}
	return converted;
}
fn find_colours(data: &Vec<u8>) -> Vec<u8> {
	let mut result: Vec<u8> = vec![];
	for i in 0..data.len() {
		let mut found = false;
		for c in 0..result.len() {
			if data[i] == result[c] {
				found = true; break;
			}
		}
		if !found {
			result.push(data[i]);
		}
	}
	return result;
}
fn match_palette() {

}

fn main() {
	let arguments = std::env::args();
	let arguments = arguments::parse(arguments).unwrap();
	/*let dump_bitmap = arguments.get::<bool>("dump-bitmap");
	let dump_bitmap = match dump_bitmap {
		None => false;
		(_) => true;
	}*/ let dump_bitmap = false; // FIXME

	assert_eq!(arguments.orphans.len(), 2,
		"Usage: {} input.png output.pic", arguments.program);
	let input_file = (&arguments.orphans[0]).to_string();
	let output_file = (&arguments.orphans[1]).to_string();
	println!("Input file: {}",input_file);
	println!("Output file: {}",output_file);

	let decoded = load_png(input_file);
	let width = decoded.0;
	let height = decoded.1;
	let data = decoded.2;
	println!("{}x{}, {} bytes.", width, height, data.len());
	let paletted = to_indexed(&data);
	println!("Paletted image size: {}",paletted.len());
	if dump_bitmap {
		for y in 0..height {
			for x in 0..width {
				let i = (y*width+x) as usize;
				let ch = paletted[i];
				print!("{:02X}",ch);
			}
			println!();
		}
	}
	let found_colours = find_colours(&paletted);
	println!("Found {} colours: {:?}",found_colours.len(),found_colours);
}
