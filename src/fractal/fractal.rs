use rayon::prelude::*;

pub trait Fractal {
    fn get_width(&self) -> u32;

    fn get_height(&self) -> u32;

    fn get_nb_iterations(&self) -> u32;

    fn draw(&self, buf: &mut [u8], pixel_size: u32) {
        let x1 = -2.1;
        let x2 = 0.6;
        let y1 = -1.2;
        let y2 = 1.2;
        let zoom_x = self.get_width() as f64 / (x2 - x1);
        let zoom_y = self.get_height() as f64 / (y2 - y1);
        let nb_iterations = self.get_nb_iterations();

        buf.chunks_mut((self.get_width() * pixel_size) as usize)
            .enumerate()
            .for_each(|(line, slice)| {
                slice
                    .chunks_exact_mut(pixel_size as usize)
                    .enumerate()
                    .for_each(|(p, pixel)| Self::draw_pixel(pixel, p, line, zoom_x, zoom_y, x1, y1, nb_iterations));
            });
    }

    fn par_draw(&self, buf: &mut [u8], pixel_size: u32) {
        let x1 = -2.1;
        let x2 = 0.6;
        let y1 = -1.2;
        let y2 = 1.2;
        let zoom_x = self.get_width() as f64 / (x2 - x1);
        let zoom_y = self.get_height() as f64 / (y2 - y1);
        let nb_iterations = self.get_nb_iterations();

        buf.par_chunks_mut((self.get_width() * pixel_size) as usize)
            .enumerate()
            .for_each(|(line, slice)| {
                slice
                    .chunks_exact_mut(pixel_size as usize)
                    .enumerate()
                    .for_each(|(p, pixel)| Self::draw_pixel(pixel, p, line, zoom_x, zoom_y, x1, y1, nb_iterations));
            });
    }

    fn draw_pixel(pixel: &mut [u8], x: usize, y: usize, zoom_x: f64, zoom_y: f64, x1: f64, y1: f64, nb_iterations: u32);
}
