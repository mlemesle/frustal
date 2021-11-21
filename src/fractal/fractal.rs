pub trait Fractal {
    fn draw(&self, buf: &mut [u8], pixel_size: u32);
}
