
Turbo Pascal Moderniser
=======================

This repository contains some of the tools that are going to be extremely
handy when developing things for Turbo Pascal 7.0. These were done primarily
for my own needs.

The tools were written in C++ using Visual Studio 2015 in as portable code
as possible, though there are still Visual Studio-isms here that mean
it won't build straight out of box or something.

PNG2BGI
-------

This program will convert these newfangled "PNG" files, whatever they
are, to raw BGI bitmaps that can be drawn with GRAPH.TPU's PutImage()
procedure.

BGI2TPU (not started yet)
-------

This will wrap BGI bitmaps into constants within Turbo Pascal units.
It takes a manifest and the BGI files referenced there in and spit
out a .pas source file. Add it as part of the project, compile it to a
.tpu using TP's own compiler, and you can use PutImage() to stick that
stuff on screen in your own program. Essentially, this tool lets you pack
the images into executables with absolutely no hassle at all.
