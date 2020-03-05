#!/usr/bin/env python3
# python/rust brute force comparison

from numpy import linspace
from sys import stdout

w = 1024
stdout.buffer.write(b'P4\n%i %i' % (w,w))
for y in linspace(-1.3, 1.3, w):
    byte = bit_idx = 0
    for x in linspace(-1.8, 0.8, w):
        c = z = complex(x, y)
        for _ in range(50):
            if abs(z) > 4: break
            z = z*z + c
        else:
            byte += 128>>bit_idx
        bit_idx += 1
        if bit_idx >= 8:
            stdout.buffer.write(byte.to_bytes(1, 'big'))
            byte = bit_idx = 0
