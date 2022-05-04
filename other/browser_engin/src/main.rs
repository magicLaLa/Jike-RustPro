use std::{
    io::{Read, BufWriter},
    fs::File,
};
use image;

use browser_engine::{layout::{Dimensions, self}, html, css, styles, painting};

fn read_source(file_path: &str) -> String {
    let mut str = String::new();
    if let Ok(mut data) = File::open(file_path) {
        data.read_to_string(&mut str).unwrap();
    }
    str
}

fn main() {
    let html = read_source("./other/browser_engin/examples/test.html");
    let css = read_source("./other/browser_engin/examples/test.css");

    let mut viewport = Dimensions::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = styles::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let out_file_name = "output.png";

    let canvas = painting::paint(&layout_root, viewport.content);
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
        let color = canvas.pixels[(y * w + x) as usize];
        image::Pixel::from_channels(color.r, color.g, color.b, color.a)
    });
    match image::DynamicImage::ImageRgb8(img).save_with_format(format!("./other/browser_engin/examples/{}", out_file_name), image::ImageFormat::Png) {
        Ok(_) => {
            println!("成功！");
        },
        Err(e) => {
            println!("保存失败: {:#?}", e);
        },
    }
}
