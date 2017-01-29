#include "stdafx.h"
#include "lodepng.h"
#include <iostream>
#include <cstdlib>
#include <vector>

int main(int argc, char *argv[])
{
	std::vector<unsigned char> image; //the raw pixels
	unsigned width = 0;
	unsigned height = 0;
	//unsigned cols = 0;

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
	}

	return 0;
}
