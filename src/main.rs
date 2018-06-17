extern crate bmp;

use bmp::{Pixel, Image};

fn main() {
    let white = Pixel::new(255, 255, 255);
    let red = Pixel::new(255, 0, 0);

    let mut img = Image::new(100, 100);

    img.draw_line(13, 20, 80, 40, white); 
    img.draw_line(20, 13, 40, 80, red); 
    img.draw_line(80, 40, 13, 20, red);

    let _ = img.save("output.bmp");
}

trait ImageMethods {
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, pixel: Pixel);
}

impl ImageMethods for Image {
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, pixel: Pixel) {
        let mut x0 = x0;
        let mut x1 = x1;
        let mut y0 = y0;
        let mut y1 = y1;

        let steep = (y1 - y0).abs() > (x1 - x0).abs();
        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let gradient = dy as f64 / dx as f64;
        
        let mut y = y0;
        let mut e = gradient - 1.0;

        for x in 0..x1 {
            if steep {
                self.set_pixel(y as u32, x as u32, pixel);
            } else {
                self.set_pixel(x as u32, y as u32, pixel);
            }
            if e >= 0.0 {
                y += 1;
                e -= 1.0;
            }
            e += gradient;
        }
    }   
}