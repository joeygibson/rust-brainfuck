## rust-brainfuck

A simple [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) compiler in [Rust](https://www.rust-lang.org/),
ported from [Go](https://golang.org) from an article by Thorsten Ball in 
[this blog post](https://thorstenball.com/blog/2017/01/04/a-virtual-brainfuck-machine-in-go/).

## Building

Just run `cargo build` and you will get a `rust-brainfuck` executable. If you want any 
kind of performance from it, run `cargo build --release` to get an optimized build. 

## Running

Pass in the name of the file you want to execute

## Execution time

Running an optimized build against `tests/mandelbrot.b` took **9.55s**. Running a
debug build on the same file took **158.11s**. 


