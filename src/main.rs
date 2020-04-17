use std::fs::OpenOptions;
use std::io::prelude::*;

pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
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
}
