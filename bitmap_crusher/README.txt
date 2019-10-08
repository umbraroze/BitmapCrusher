
Bitmap Crusher
==============

This program will convert PNG bitmaps to BGI (Borland Graphics Interface)
format in Turbo Pascal 7 for DOS. The first phase of development is meant
to make the application work with CGA 4 colour mode bitmaps, for the
retroesque indie games I'm developing.

The crusher supports manifest files that determine the names and source file
names for the bitmaps, so entire project's graphics can be converted at once.
The program will output images as either binary files or as a Turbo Pascal
unit source code (so graphics can be nicely packed inside your executable).


BGI bitmap documentation
------------------------

Turbo Pascal BGI doesn't support loading or saving images directly;
image data can practically come from anywhere you want.

In BGI, there are three procedures for handling "image regions" (i.e.
sub-bitmaps, tiles or sprites). They can, of course, be used to
fetch entire screens too, but the PutImage procedure has interesting
options for outputting these image regions (including XORing for
rudimentary graphics trickery). The other two procedures are GetImage
(fetch an image region to a buffer) and ImageSIze (determine the
storage space needed for a given image region).

There's no actual strict technical definition of BGI bitmap structure in
the Turbo Pascal documentation. My guess is that BGI developers never
intended the bitmaps to be manipulated by anything else besides BGI
itself.

Here's what "Turbo Pascal Version 7.0 Programmer's Reference" has to say
about two BGI procedures, GetImage (p. 70) and ImageSize (p. 88).

GetImage:
  BitMap is an untyped parameter that must be greater than or equal to 6
  plus the amount of area defined by the region. The first two words
  of BitMap store the width and height of the region. The third word
  is reserved. The remaining part of BitMap is used to save the bit image
  itself. [...] The memory required to save the region must be less than
  64K.

ImageSize:
  ImageSize determines the number of bytes necessary for GetImage to save
  the specified region of the screen. The image size includes space for
  several words. The first stores the width of the region, and the second
  stores the height. The next words store the attributes of the image
  itself. The last word is reserved.

In other words, my best guess of what the BGI bitmap looks like is this:

  u16 width
  u16 height
  u16 mode-dependant attribute flags, maybe, maybe not???
  u16 reserved
  image data...

I'm particularly confused by contradicting definitions in these function
descriptions; GetImage doesn't mention anything about the image attributes.
My only real option is to practically try out the function and see what it
outputs.

Image data itself shouldn't be difficult to figure out: it's identical to how
the image data is stored in video memory, and is therefore video mode
dependant. There's also nothing in the header that would indicate the actual
video mode (and therefore how many attribute words are needed), so I assume
what happens is that BGI says "I know what mode flags this mode needs and
expect those to be there. If they are not, well, enjoy your mangled bitmap
that I stuck on the screen."

The raw data will be in usual format. For example, 4 colour CGA will
need 2 bits per pixel, so a byte will fit 4 pixels. I don't know what
happens if we're dealing with non-multiples-of-4 image region widths.
Strangely aligned data, most likely.



CGA colour palettes
-------------------

Palette 0 Low:   0,  2,  4,  6 (black, green, red, brown)
          High:  0, 10, 12, 14 (black, lt.green, lt.red, yellow)
        1 Low:   0,  3,  5,  7 (black, cyan, magenta, lt.gray)
          High:  0, 11, 13, 15 (black, lt.cyan, lt.magenta, white)

		{ 0x00, 0x00, 0x00 }  // 0 black
		{ 0x00, 0x00, 0xAA }  // 1 blue
		{ 0x00, 0xAA, 0x00 }  // 2 green
		{ 0x00, 0xAA, 0xAA }  // 3 cyan
		{ 0xAA, 0x00, 0x00 }  // 4 red
		{ 0xAA, 0x00, 0xAA }  // 5 magenta
		{ 0xAA, 0x55, 0x00 }  // 6 brown
		{ 0xAA, 0xAA, 0xAA }  // 7 light gray
		{ 0x55, 0x55, 0x55 }  // 8 dark gray
		{ 0x55, 0x55, 0xFF }  // 9 light blue
		{ 0x55, 0xFF, 0x55 }  // 10 light green
		{ 0x55, 0xFF, 0xFF }  // 11 light cyan
		{ 0xFF, 0x55, 0x55 }  // 12 light red
		{ 0xFF, 0x55, 0xFF }  // 13 light magenta
		{ 0xFF, 0xFF, 0x55 }  // 14 yellow
		{ 0xFF, 0xFF, 0xFF }  // 15 white
