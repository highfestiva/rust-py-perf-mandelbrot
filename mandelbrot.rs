// rust/python brute force comparison

extern crate num;

use num::Complex;
use std::io::Write;

fn main() {
	let w = 1024;
	println!("P4\n{} {}", w, w);
	for ys in 0..w {
		let y = (ys as f32) * 2.6 / ((w-1) as f32) - 1.3;
		let mut byte = 0 as u8;
		let mut bit_idx = 0 as u8;
		for xs in 0..w {
			let x = (xs as f32) * 2.6 / ((w-1) as f32) - 1.8;
			let c = Complex { re: x, im: y };
			let mut z = c;
			let mut solid = true;
			for _ in 0..50 {
				if z.norm() > 4.0 {
					solid = false;
					break
				}
				z = z*z + c
			}
			if solid {
				byte |= 128 >> bit_idx;
			}
			bit_idx += 1;
			if bit_idx >= 8 {
				let bytes = [byte; 1];
				std::io::stdout().write(&bytes).unwrap();
				byte = 0;
				bit_idx = 0;
			}
		}
	}
}
