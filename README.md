# Python/Rust mandelbrot brute force performance comparison
I want to learn rust, and this is my first stumbling step. So I wrote a super-simple
minimal mandelbrot implementation in Python in ~20 loc and just ported it as
straight-offish as I could to Rust. Disclaimer: I totally suck at Rust.

## Output
The output is a PBM mandelbrot of 1024x1024 pixels. For some strange reason they're
not exactly identical, the python version is shifted slightly to the left (although
they both start at x=-1.8). Weird.

![sample](https://raw.githubusercontent.com/highfestiva/rust-py-perf-mandelbrot/master/mandelbrot.jpg)

## Performance
Rust was more than 30 times faster on 1M data points. Even compiling+running was a
ton faster than Python. Had expected an even higher speedup as the
[current](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/mandelbrot.html)
language shootout implementations says Rust is 153 times faster than Python 3,
but then I realized that those measurements are on 256M data points. So if you're
doing something not-so-out-of-the-ordinary, 1M data points are probably closer to
the truth, and a 30x speedup is probably also closer to what you'll get.

## Code size
Rust code became approximately 75% longer, and had an indentation extra of course.
That's discounting the build file.

## Data

````bash
$ time ./mandelbrot.py > py.pbm

real    0m4.557s
user    0m0.000s
sys     0m0.000s


$ time cargo build --release && target/release/mandelbrot.exe > rust.pbm
   Compiling mandelbrot v0.0.1 (C:\RnD\py\mandelbrot)
    Finished release [optimized] target(s) in 0.57s

real    0m0.623s
user    0m0.015s
sys     0m0.000s


$ time target/release/mandelbrot.exe > rust.pbm

real    0m0.131s
user    0m0.000s
sys     0m0.015s
````
