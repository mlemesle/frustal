use super::fractal::Fractal;

pub struct Mandelbrot {
    pub width: u32,
    pub height: u32,
    pub nb_iterations: u32,
}

impl Mandelbrot {
    pub fn new(width: u32, height: u32, nb_iterations: u32) -> Self {
        Mandelbrot { width, height, nb_iterations }
    }
}

impl Fractal for Mandelbrot {
    fn draw(&self, buf: &mut [u8], pixel_size: u32) {
        let x1 = -2.1;
        let x2 = 0.6;
        let y1 = -1.2;
        let y2 = 1.2;
        let zoom_x = self.width as f64 / (x2 - x1);
        let zoom_y = self.height as f64 / (y2 - y1);

        for x in 0..self.width {
            for y in 0..self.height {
                let c_r = x as f64 / zoom_x + x1;
                let c_i = y as f64 / zoom_y + y1;
                let mut z_r = 0.0;
                let mut z_i = 0.0;
                let mut i = 0;

                while z_r * z_r + z_i * z_i < 4.0 && i < self.nb_iterations {
                    let tmp = z_r;
                    z_r = z_r*z_r - z_i*z_i + c_r;
                    z_i = 2.0*z_i*tmp + c_i;
                    i += 1;
                }

                let index = ((y * self.height + x) * pixel_size) as usize;
                let color = u32::MAX / i;
                buf[index] = ((color & 0xFF0000) >> 16) as u8;
                buf[index + 1] = ((color & 0xFF00) >> 8) as u8;
                buf[index + 2] = (color & 0xFF) as u8;
            }
        }
    }
}