
mod cga_image;
use cga_image::Image;
extern crate arguments;

fn usage(program: &str) {
	eprintln!();
	eprintln!("Usage: {} [--dump-bitmap] [--ansi-dump] input.png output.pic",
		program);
	eprintln!("  --dump-bitmap true: Also print out the bitmap as bytes.");
	eprintln!("  --ansi-dump true: Use ANSI terminal colour with the dump.");
	eprintln!();
	panic!();
}

fn main() {
	// Parse command line arguments
	let arguments = std::env::args();
	let arguments = arguments::parse(arguments).unwrap();
	let dump_bitmap = arguments.get::<bool>("dump-bitmap");
	let dump_bitmap = match dump_bitmap {
		None => false, // default value
		_ => dump_bitmap.unwrap()
	};
	let ansi_dump = arguments.get::<bool>("ansi-dump");
	let ansi_dump = match ansi_dump {
		None => false, // default value
		_ => ansi_dump.unwrap()
	};
	if arguments.orphans.len() != 2 {
		usage(&arguments.program);
	}
	let input_file = (&arguments.orphans[0]).to_string();
	let output_file = (&arguments.orphans[1]).to_string();
	println!("Input file: {}",input_file);
	println!("Output file: {}",output_file);

	// Decode image
	let image = Image::from_png(input_file);
	println!("{}x{}, {} bytes.", image.width, image.height, image.rgba_data.len());
	println!("Paletted image size: {}",image.p_data.len());
	if image.p_data.len() % 4 != 0 {
		panic!("Can currently only handle images that align in 4 pixels.");
	}
	// Dump bitmap if so desired
	if dump_bitmap {
		image.dump_bitmap(ansi_dump);
	}
	// Find all of the colours
	let found_colours = image.find_colours();
	println!("Found {} colours: {:?}",found_colours.len(),found_colours);
	if found_colours.len() > 4 {
		panic!("Can only deal with 4 colour images.");
	}
	// Find the palette
	let pidx = Image::match_palette(found_colours);
	match pidx {
		None => panic!("The image has an invalid combo of colours in palette."),
		Some(x) => {
			println!("Palette used in image: {}",cga_image::MODES[x]);
		}
	}
}
