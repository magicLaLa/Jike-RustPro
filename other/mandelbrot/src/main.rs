extern crate crossbeam;
extern crate image;
extern crate num;
use image::{png::PNGEncoder, ColorType};
use mandelbrot::{parse_complex, parse_pair, pixel_to_point};
use num::Complex;
use std::fs::File;
use std::io::Write;

/// 确定 c 是否属于 曼得布洛特集合，最多循环 limit 次
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/// 将矩形区域内的 曼得布洛特集合 渲染为 像素保存在 缓中区
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                Some(count) => 255 - count as u8,
                None => 0,
            }
        }
    }
}

/// 把缓冲区中的 pixels(大小由 bounds 指定),写到名为 filename 的文件中
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    println!("output: {:?}", output);
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: mandelbrot File Pixles upperLeft LowerRight"
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimemsions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
