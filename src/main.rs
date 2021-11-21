mod fractal;

use std::fs::File;
use std::io::{BufWriter};

use fractal::mandelbrot::Mandelbrot;
use fractal::fractal::Fractal;

fn main() {
    let width = 2500;
    let height = 2500;
    let pixel_size = 3;
    let m = Mandelbrot::new(width, height, 1500);
    let buf: &mut [u8] = &mut vec![0; (width*height*pixel_size) as usize];
    m.draw(buf, pixel_size);
    let file = File::create("mandel.png").unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(buf).unwrap();
}