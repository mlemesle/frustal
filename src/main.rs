mod fractal;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

use fractal::mandelbrot::Mandelbrot;
use fractal::fractal::Fractal;
use inline_python::python;

fn bench(m: Mandelbrot, buf: &mut [u8], pixel_size: u32) {
    let nb_run = 30;
    let mut st_time_elapsed = vec![0.0; nb_run];

    println!("Starting {} single thread computation", nb_run);
    for run in 0..nb_run {
        println!("S.T : {}", run);
        let current_time = Instant::now();
        m.draw(buf,pixel_size);
        st_time_elapsed[run] = current_time.elapsed().as_secs_f32();
    }

    let mut mt_time_elapsed = vec![0.0; nb_run];

    println!("Starting {} multi thread computation", nb_run);
    for run in 0..nb_run {
        println!("M.T : {}", run);
        let current_time = Instant::now();
        m.par_draw(buf,pixel_size);
        mt_time_elapsed[run] = current_time.elapsed().as_secs_f32();
    }

    println!("S.T time array : {:?}", st_time_elapsed);
    println!("M.T time array : {:?}", mt_time_elapsed);

    python! {
        import matplotlib.pyplot as plt

        plt.suptitle("Comparison between single and multi thread mandel")

        plt.subplot(2, 1, 1)
        plt.title("Measurements")
        plt.plot('st_time_elapsed, linestyle="--", marker='o', label="mono thread")
        plt.plot('mt_time_elapsed, linestyle="--", marker='o', label="multi thread")
        plt.xlabel("Run #")
        plt.ylabel("seconds")

        plt.show()
    }
}

fn draw_png(m: Mandelbrot, buf: &mut [u8], pixel_size: u32) {
    m.draw(buf, pixel_size);
    let file = File::create("mandel.png").unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, m.width, m.height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(buf).unwrap();
}

fn main() {
    let width = 2500;
    let height = 2500;
    let pixel_size = 3;
    let m = Mandelbrot::new(width, height, 1500);
    let buf: &mut [u8] = &mut vec![0; (width*height*pixel_size) as usize];
    
    bench(m, buf, pixel_size);
}