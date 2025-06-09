
# Turbo Pascal Moderniser

This repository contains some of the tools that are going to be extremely
handy when developing things for Turbo Pascal 7.0. These were done primarily
for my own needs.

This is a very brainfarty old project that nevert got completed, and I
hope to complete it now.

The tools are (re)written in Rust. The C++ edition is pretty much abandoned
and will be gone when I get the ducks in row.

## bitmap_crusher

This program will convert these newfangled "PNG" files, whatever they
are, to raw BGI bitmaps that can be drawn with GRAPH.TPU's PutImage()
procedure.

## bitmap_uniter (or whatever, it's not started)

This will wrap BGI bitmaps into constants within Turbo Pascal units.
It takes a manifest and the BGI files referenced there in and spit
out a .pas source file. Add it as part of the project, compile it to a
.tpu using TP's own compiler, and you can use PutImage() to stick that
stuff on screen in your own program. Essentially, this tool lets you pack
the images into executables with absolutely no hassle at all.
