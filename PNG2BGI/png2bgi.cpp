// Visual C++ stuff
#include "stdafx.h"

// LodePNG library
#include "lodepng.h"

// Standard stuff
#include <iostream>
#include <iomanip>
#include <sstream>
#include <string>
#include <cstdlib>
#include <cstdint>
#include <vector>

typedef std::vector<unsigned char> image_data;

class Color {
public:
	Color();
	Color(uint8_t r, uint8_t g, uint8_t b);
	~Color();
	bool operator==(Color c);
	std::string to_string();
protected:
	uint8_t r, g, b;
};
Color::Color() :
	r(0), g(0), b(0) { }
Color::Color(uint8_t r, uint8_t g, uint8_t b) :
	r(r), g(g), b(b) { }
Color::~Color()
{
}
bool Color::operator==(Color c)
{
	if (r == c.r && g == c.g && b == c.b)
		return true;
	return false;
}
std::string Color::to_string()
{
	std::stringbuf o;
	std::ostream os(&o);
	// Sorry this formatting sucks, C++ stream formatting
	// is beyond my comprehension at the moment
	os << std::setw(2) //<< std::setfill('0')
		<< std:: hex
		<< "#" << (int)r << " " << (int)g << " " << (int)b
		<< std::setw(1) << std::dec
		<< " (" << (int)r << "," << (int)g << "," << (int)b << ")";
	return o.str();
}
typedef std::vector<Color> Palette;
const Palette CGA_P0LO_PALETTE = {
	{ 0x00, 0x00, 0x00 }, // 0 black
	{ 0x00, 0xAA, 0xAA }, // 3 cyan
	{ 0xAA, 0x00, 0xAA }, // 5 magenta
	{ 0xAA, 0xAA, 0xAA }  // 7 light gray
};
const Palette CGA_P0HI_PALETTE = {
	{ 0x00, 0x00, 0x00 }, //  0 black
	{ 0x55, 0xFF, 0xFF }, // 11 light cyan
	{ 0xFF, 0x55, 0xFF }, // 13 light magenta
	{ 0xFF, 0xFF, 0xFF }  // 15 white
};
const Palette CGA_P1LO_PALETTE = {
	{ 0x00, 0x00, 0x00 }, // 0 black
	{ 0x00, 0xAA, 0x00 }, // 2 green
	{ 0xAA, 0x00, 0x00 }, // 4 red
	{ 0xAA, 0x55, 0x00 }  // 6 brown
};
const Palette CGA_P1HI_PALETTE = {
	{ 0x00, 0x00, 0x00 }, //  0 black
	{ 0x55, 0xFF, 0x55 }, // 10 light green
	{ 0xFF, 0x55, 0x55 }, // 12 light red
	{ 0xFF, 0xFF, 0x55 }  // 14 yellow
};
enum PaletteTypes {
	CGA_P0LO = 0, CGA_P0HI = 1,
	CGA_P1LO = 2, CGA_P1HI = 3
};
/*
const std::vector<*Palette> PALETTES = {
	&CGA_P0LO_PALETTE,
	&CGA_P0HI_PALETTE,
	&CGA_P1LO_PALETTE,
	&CGA_P1HI_PALETTE
};
*/

class BGIBitmap {
public:
	BGIBitmap();
	~BGIBitmap();
	void encode(image_data image, int palette_size);
private:
	// Fields that go out to the file
	uint16_t width;
	uint16_t height;
	unsigned char *data;
	// Other stuff
	int palette_count();
};
BGIBitmap::BGIBitmap() :
	width(0),
	height(0),
	data(NULL) {}
BGIBitmap::~BGIBitmap() {
	if (data != NULL)
		free(data);
}
void BGIBitmap::encode(image_data image, int palette_size) {
}
int BGIBitmap::palette_count() {
	return 0;
}

int main(int argc, char *argv[]) {
	image_data image; //the raw pixels
	unsigned width = 0;
	unsigned height = 0;
	std::vector<Color> seen_colors;

	if (argc != 3) {
		std::cerr << "Usage: " << argv[0] << " input.png output.pic" << std::endl;
		exit(EXIT_FAILURE);
	}
	const char* inputfilename = argv[1];
	const char* outputfilename = argv[2];

	std::cout << "Input: " << inputfilename << std::endl
		<< "Output: " << outputfilename << std::endl;

	// Decode the PNG file
	unsigned error = lodepng::decode(image, width, height, inputfilename);
	if (error) std::cout << "PNG decoder error " << error << ": "
		<< lodepng_error_text(error) << std::endl;

	// the pixels are now in the vector "image", 4 bytes per pixel,
	// ordered RGBARGBA..., use it as texture, draw it, ...

	std::cout << "File decoded successfully." << std::endl;
	std::cout << "Size: " << width << "x" << height << " pixels." << std::endl;

	for (unsigned i = 0; i < width*height; i += 4) {
		Color c((uint8_t)image[i], (uint8_t)image[i+1], (uint8_t)image[i+2]);
		if (seen_colors.size() == 0)
			seen_colors.push_back(c); // If no colors are seen, add it as first
		else {
			// Loop over seen colors and see if any of them match
			bool seen = false;
			for (unsigned j = 0; j < seen_colors.size(); j++) {
				if (c == seen_colors[j]) {
					seen = true;
					break;
				}
			}
			// If it's a new color, add it
			if (!seen)
				seen_colors.push_back(c);
		}
	}
	std::cout << "Image has " << seen_colors.size() << " colors" << std::endl;
	for (unsigned i = 0; i < seen_colors.size(); i++) {
		std::cout << " - " << seen_colors[i].to_string() << std::endl;
	}

	return 0;
}

