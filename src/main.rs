use image::{DynamicImage, ImageBuffer, ImageFormat, Pixel};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod painter;
pub mod style;

fn main() {
    /* HTML */
    let mut html_source = "".to_string();
    let mut options = OpenOptions::new();
    let mut file = options.read(true).open("./examples/test.html").unwrap();
    file.read_to_string(&mut html_source)
        .ok()
        .expect("cannot read html file");

    let root_node = html::parse(html_source);
    println!("HTML: {:?} ", root_node);

    /* CSS */
    let mut css_source = "".to_string();
    let mut options = OpenOptions::new();
    let mut file = options.read(true).open("./examples/test.css").unwrap();
    file.read_to_string(&mut css_source)
        .ok()
        .expect("cannot read css file");

    let stylesheet = css::parse(css_source);
    println!("Stylesheet: {:?}: ", stylesheet);

    let mut viewport: layout::Dimensions = ::std::default::Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let style_tree = style::style_tree(&root_node, &stylesheet);
    let layout_tree = layout::layout_tree(&style_tree, viewport);
    println!("{:?}", layout_tree);

    /* File output */
    let canvas = painter::paint(&layout_tree, viewport.content);
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let img = ImageBuffer::from_fn(w, h, move |x, y| {
        let color = canvas.pixels[(y * w + x) as usize];
        Pixel::from_channels(color.r, color.g, color.b, color.a)
    });
    let png_img = DynamicImage::ImageRgba8(img);
    let path = Path::new("public/output.png");
    png_img.save_with_format(path, ImageFormat::Png).unwrap();
}
