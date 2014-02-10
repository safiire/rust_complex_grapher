rust_complex_grapher
====================

This is a program which can generate two-dimensional graphs of complex valued functions.  It is written Rust 0.10pre, cause I'm trying to keep up with the master branch :)

This is a work in progress, in order to change the equation graphed you must edit the main.rs file to change the value
of z.

I intend to read equations from the commandline and parse them in the near future.

It produces uncompressed ppm files.

Hope you can learn something from it :)


Example Output
====================

Here is the output for 

f(z) = z^5 + 1

![F(z) = Z^5 + 1](H(z)-=-z--1---z^-1.png "F(z) = Z^5 + 1")

f(z) = z - 1 - Z^-1

![f(z) = z - 1 - Z^-1](H(z)-=-z--1---z^-1.png "f(z) = z - 1 - Z^-1")

f(z) = sin(z)

![sin(z)](sin(z).png "sin(z)")
