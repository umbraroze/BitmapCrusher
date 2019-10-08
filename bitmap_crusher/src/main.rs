
use std::fs::File;

// TODO: Make the struct fancily comparable with just an ==
// TODO: Some kind of a function to convert the colour to a string.
struct Colour {
    r: u8,
    g: u8,
    b: u8
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
    Colour { r: 0x00, g: 0x00, b: 0x00 },  // 0 black
	Colour { r: 0x00, g: 0x00, b: 0xAA },  // 1 blue
	Colour { r: 0x00, g: 0xAA, b: 0x00 },  // 2 green
	Colour { r: 0x00, g: 0xAA, b: 0xAA },  // 3 cyan
	Colour { r: 0xAA, g: 0x00, b: 0x00 },  // 4 red
	Colour { r: 0xAA, g: 0x00, b: 0xAA },  // 5 magenta
	Colour { r: 0xAA, g: 0x55, b: 0x00 },  // 6 brown
	Colour { r: 0xAA, g: 0xAA, b: 0xAA },  // 7 light gray
	Colour { r: 0x55, g: 0x55, b: 0x55 },  // 8 dark gray
	Colour { r: 0x55, g: 0x55, b: 0xFF },  // 9 light blue
	Colour { r: 0x55, g: 0xFF, b: 0x55 },  // 10 light green
	Colour { r: 0x55, g: 0xFF, b: 0xFF },  // 11 light cyan
	Colour { r: 0xFF, g: 0x55, b: 0x55 },  // 12 light red
	Colour { r: 0xFF, g: 0x55, b: 0xFF },  // 13 light magenta
	Colour { r: 0xFF, g: 0xFF, b: 0x55 },  // 14 yellow
	Colour { r: 0xFF, g: 0xFF, b: 0xFF }   // 15 white
];

fn to_indexed(data: Vec<u8>) -> Vec<u8> {
    assert_eq!(data.len() % 4, 0,
        "Data size not divisible by 4 (not RGBA image???)");
    let l = data.len()/4;
    let mut converted = vec![0 as u8; l];
    for n in 0..l-1 {
        let dp = n * 4;
        let ra = data[dp];
        let ga = data[dp+1];
        let ba = data[dp+2];
        let p = Colour {r: ra, g: ga, b: ba};

        let mut cidx: u8 = 16;
        // FIXME: Why 0 to 16 and not 15? Something funny is going on here.
        for colour in 0..16 {
            let c = &PALETTE[colour];
            //println!("{}",colour);
            if c.r == p.r &&
                c.g == p.g &&
                c.b == p.b {
                cidx = colour as u8;
                break;
            }
        }
        //println!(" => {}",cidx);
        assert_ne!(cidx,16,
            "Pixel {} offset {}, colour not in palette: #{:02X}{:02X}{:02X}",
            n, dp, p.r,p.g,p.b);
        converted[n] = cidx;
    }
    return converted;
}

fn main() {
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();

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
    let paletted = to_indexed(data);
    println!("Paletted image size: {}",paletted.len());
    for y in 0..height-1 {
        for x in 0..width-1 {
            print!("{:02X}",paletted[(y*height+x) as usize]);
        }
        println!("");
    }
}
