use super::fractal::Fractal;

pub struct Mandelbrot {
    pub width: u32,
    pub height: u32,
    pub nb_iterations: u32,
}

impl Mandelbrot {
    pub fn new(width: u32, height: u32, nb_iterations: u32) -> Self {
        Mandelbrot {
            width,
            height,
            nb_iterations,
        }
    }
}

impl Fractal for Mandelbrot {

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn get_nb_iterations(&self) -> u32 {
        self.nb_iterations
    }

    fn draw_pixel(pixel: &mut [u8], x: usize, y: usize, zoom_x: f64, zoom_y: f64, x1: f64, y1: f64, nb_iterations: u32) {
        let c_r = x as f64 / zoom_x + x1;
        let c_i = y as f64 / zoom_y + y1;
        let mut z_r = 0.0;
        let mut z_i = 0.0;
        let mut i = 0;

        while z_r * z_r + z_i * z_i < 4.0 && i < nb_iterations {
            let tmp = z_r;
            z_r = z_r * z_r - z_i * z_i + c_r;
            z_i = 2.0 * z_i * tmp + c_i;
            i += 1;
        }

        if i == nb_iterations {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
        } else {
            let color = u32::MAX / i;
            pixel[0] = (color & 0xFF0000 >> 16) as u8;
            pixel[1] = (color & 0xFF00 >> 8) as u8;
            pixel[2] = color as u8;
        }
    }
}
