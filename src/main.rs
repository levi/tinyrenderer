extern crate bmp;

use bmp::{Pixel, Image};

fn main() {
    let red = Pixel::new(255, 0, 0);

    let mut img = Image::new(100, 100);
    img.set_pixel(52, 41, red);
    let _ = img.save("output.bmp");
}
