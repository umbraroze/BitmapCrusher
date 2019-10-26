
mod cga_image;
use cga_image::Image;

fn main() {
	let arguments = std::env::args();
	let arguments = arguments::parse(arguments).unwrap();
	/*let dump_bitmap = arguments.get::<bool>("dump-bitmap");
	let dump_bitmap = match dump_bitmap {
		None => false;
		(_) => true;
	}*/ let dump_bitmap = false; // FIXME

	if arguments.orphans.len() != 2 {
		panic!("Usage: {} input.png output.pic", arguments.program);
	}
	let input_file = (&arguments.orphans[0]).to_string();
	let output_file = (&arguments.orphans[1]).to_string();
	println!("Input file: {}",input_file);
	println!("Output file: {}",output_file);

	let image = Image::from_png(input_file);
	println!("{}x{}, {} bytes.", image.width, image.height, image.rgba_data.len());
	println!("Paletted image size: {}",image.p_data.len());
	if dump_bitmap {
		for y in 0..image.height {
			for x in 0..image.width {
				let i = (y*image.width+x) as usize;
				let ch = image.p_data[i];
				print!("{:02X}",ch);
			}
			println!();
		}
	}
	let found_colours = image.find_colours();
	println!("Found {} colours: {:?}",found_colours.len(),found_colours);
	if found_colours.len() > 4 {
		panic!("Can only deal with 4 colour images.");
	}
}
