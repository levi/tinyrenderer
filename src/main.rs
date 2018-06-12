extern crate bmp;

use bmp::{Pixel, Image};

fn main() {
    let white = Pixel::new(255, 255, 255);
    let red = Pixel::new(255, 0, 0);

    let mut img = Image::new(100, 100);
    img.set_pixel(52, 41, red);
    img.draw_line(2.0, 20.0, 40.0, 55.0, white);
    let _ = img.save("output.bmp");
}

trait ImageMethods {
    fn draw_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, pixel: Pixel);
}

impl ImageMethods for Image {
    fn draw_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, pixel: Pixel) {
        let mut t = 0.0;
        while t < 1.0 {
            let x = x0 * (1.0 - t) + x1 * t;
            let y = y0 * (1.0 - t) + y1 * t;
            self.set_pixel(x as u32, y as u32, pixel);
            t += 0.01;
        }
    }   
}