extern crate image;
extern crate num_complex;

use num_complex::Complex;
use std::env;

/// Define some nice color palettes
fn color_palette(pal: &str) -> [[u8; 3]; 5] {
	match pal {
		"cool" => [[0, 0, 0],
						[60, 0, 90],
						[128, 238, 255],
						[0, 100, 200],
						[0, 0, 0]],
		"plasma" => [[0, 0, 0],
						[0, 60, 150],
						[240, 255, 128],
						[200, 0, 128],
						[0, 0, 0]],
		"crystal" => [[255, 255, 255],
						[200, 255, 255],
						[150, 220, 255],
						[90, 90, 180],
						[0, 0, 0]],
		_ => [[0, 0, 0],
						[100, 100, 100],
						[150, 150, 150],
						[200, 200, 200],
						[255, 255, 255]],
	}
}

/// Returns an rgb color as `[u8; 3]` from a `count` value between
/// 0 and 1 representing how many iterations it takes for a point
/// to escape, and a `color_shift` value between 0 and 1 which allows
/// you to slide between two color palettes.
fn count_to_rgb(count: f64, color_shift: f64) -> [u8; 3] {
	// gradient colors
	let color_palette_1 = color_palette("crystal");
	let color_palette_2 = color_palette("crystal");
	// count values that mark new colors in the gradient
	let grads = [0.0, 0.32, 0.44, 0.8, 1.0];
	// find which two colors we're between, and where in between
	let mut color_ind = None;
	for ind in 0..grads.len()-1 {
		if grads[ind] <= count && count <= grads[ind+1] {
			color_ind = Some(ind);
		}
	}
	// extract values from Options
	match color_ind {
		Some(cind) => {
			// Create the correct color for each palette, then blend depending on color_shift
			let mut rgb: [u8; 3] = [0, 0, 0];
			for channel in 0..3 {
				let c1p1_c = color_palette_1[cind][channel] as f64;
				let c2p1_c = color_palette_1[cind+1][channel] as f64;
				let c1p2_c = color_palette_2[cind][channel] as f64;
				let c2p2_c = color_palette_2[cind+1][channel] as f64;
				let amt = (count - grads[cind])/(grads[cind+1] - grads[cind]);
				let p1 = c1p1_c + amt * (c2p1_c - c1p1_c);
				let p2 = c1p2_c + amt * (c2p2_c - c1p2_c);
				rgb[channel] = (p1 + color_shift*(p2-p1)) as u8;
			}
			return rgb
		},
		_ => panic!("BAD!")
	}
}

struct Params {
	max_iter: u8,
	img_width: u32,
	aspect_ratio: f64,
	window_width: f64,
	window_center: Complex<f64>,
	num_frames:  u64,
	c_init: Complex<f64>,
	c_final: Complex<f64>,
	title: String,
}

impl Params {
	fn get_c(&self, n: u64) -> Complex<f64> {
		if self.num_frames == 1 {
			self.c_init
		} else {
			let transition: f64 = (n as f64)/((self.num_frames-1) as f64);
			self.c_init + transition*(self.c_final-self.c_init)
		}
	}
}

fn construct_params(args: Vec<String>) -> Params {
	let params = Params {
		max_iter: 255,
		img_width: 192*2,
		aspect_ratio: 16.0/9.0,
		window_width: 0.64,
		window_center: Complex::new(-0.385, 0.190),
		num_frames:  1,
		c_init: Complex::new(-0.747, 0.2),
		c_final: Complex::new(-0.747, 0.2),
		title: String::from("bean"),
	};
	params
}

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	let params = construct_params(args);
	// number of iterates per z-value
	let max_iter = params.max_iter;
	// img dimensions
	let width = params.img_width;
	let height = (params.img_width as f64 / params.aspect_ratio) as u32;
	// size of window in julia set
	let win_width = params.window_width;
	let scale = win_width/width as f64;

	let mut buf = image::RgbImage::new(width, height);

	// where we put the camera in the julia set
	let centerx = params.window_center.re; // increase to move camera right
	let centery = params.window_center.im; // increase to move camera up
	// specify num_frames > 1 for animation
	let num_frames = params.num_frames;
	for n in 0..num_frames {
		// change Im(c) slightly each frame
		let c = params.get_c(n);
		// draw the picture
		for (x, y, pixel) in buf.enumerate_pixels_mut() {
			let zx = x as f64 * scale - win_width/2.0 + centerx;
			let zy = (height-y) as f64 * scale - win_width/2.0 + centery;
			let mut z = Complex::new(zx, zy);
			// number of iterates it takes for z to escape
			let mut count = 0;
			// iterate the map z^2 + c
			for x in 0..max_iter {
				if z.norm() > 2.0 {
					break
				}
				z = z*z + c;
				count = x;
			}
			// strength of attraction between julia and z, from 0 to 1
			let count: f64 = (count as f64) / 255.0;
			let color_shift = (n as f64)/(num_frames as f64);
			*pixel = image::Rgb(count_to_rgb(count, color_shift));
		}
		buf.save(format!("{}.png", params.title)).unwrap();
	}

}