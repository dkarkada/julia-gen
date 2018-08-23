extern crate image;
extern crate num_complex;

use num_complex::Complex;

/// Returns an rgb color as `[u8; 3]` from a `count` value between
/// 0 and 1 representing how many iterations it takes for a point
/// to escape, and a `color_shift` value between 0 and 1 which allows
/// you to slide between two color palattes.
fn count_to_rgb(count: f64, color_shift: f64) -> [u8; 3] {
	// gradient colors
	let color_palatte_1: [[u8; 3]; 5] = [[0, 0, 0],
										[60, 0, 90],
										[128, 238, 255],
										[0, 100, 200],
										[0, 0, 0]];
	let color_palatte_2: [[u8; 3]; 5] = [[0, 0, 0],
										[0, 60, 150],
										[240, 255, 128],
										[200, 0, 128],
										[0, 0, 0]];
	// count values that mark new colors in the gradient
	let grads = [0.0, 0.32, 0.44, 0.8, 1.0];
	// find which two colors we're between, and where in between
	let mut color1_p1 = None;
	let mut color2_p1 = None;
	let mut color1_p2 = None;
	let mut color2_p2 = None;
	let mut amt = None;
	for ind in 1..grads.len() {
		if grads[ind-1] < count && count <= grads[ind] {
			color1_p1 = Some(color_palatte_1[ind-1]);
			color2_p1 = Some(color_palatte_1[ind]);
			color1_p2 = Some(color_palatte_2[ind-1]);
			color2_p2 = Some(color_palatte_2[ind]);
			amt = Some((count - grads[ind-1])/(grads[ind] - grads[ind-1]));
		}
	}
	// extract values from Option
	let c1p1: [u8; 3];
	let c2p1: [u8; 3];
	let c1p2: [u8; 3];
	let c2p2: [u8; 3];
	let a: f64;
	match (color1_p1, color2_p1, color1_p2, color2_p2, amt) {
		(Some(c1p1t), Some(c2p1t), Some(c1p2t), Some(c2p2t), Some(z)) => {
			c1p1 = c1p1t;
			c2p1 = c2p1t;
			c1p2 = c1p2t;
			c2p2 = c2p2t;
			a = z; }
		_ => panic!("BAD!")
	}
	// Create the correct color for each palatte, then blend depending on color_shift
	let mut rgb: [u8; 3] = [0, 0, 0];
	for channel in 0..3 {
		let c1p1_c = c1p1[channel] as f64;
		let c2p1_c = c2p1[channel] as f64;
		let c1p2_c = c1p2[channel] as f64;
		let c2p2_c = c2p2[channel] as f64;
		let p1 = c1p1_c + a * (c2p1_c - c1p1_c);
		let p2 = c1p2_c + a * (c2p2_c - c1p2_c);
		rgb[channel] = (p1 + color_shift*(p2-p1)) as u8;
	}
	rgb
}

fn main() {
	// number of iterates per z-value
	let max_iter = 255;
	// width of 16:9 image
	let width = 192*10;
	// size of window in julia set
	let sz = 0.64 as f64;
	let scale = sz/width as f64;

	let mut buf = image::RgbImage::new(width, width*9/16);

	// where we put the camera in the julia set
	let centerx = -0.385; // increase to move camera left
	let centery = 0.297; // increase to move camera down
	// specify num_frames > 1 for animation
	let num_frames = 480;
	for n in 0..num_frames {
		// change Im(c) slightly each frame
		let dcy = ((n-(num_frames/2)) as f64)/400.0; //(n as f64)/2000.0;
		let c = Complex::new(-0.747, 0.2 + dcy);
		// draw the picture
		for (x, y, pixel) in buf.enumerate_pixels_mut() {
			let zx = x as f64 * scale - sz/2.0 + centerx;
			let zy = y as f64 * scale - sz/2.0 + centery;
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
		buf.save(format!("imgs/fractal{}.png", n)).unwrap();
	}

}