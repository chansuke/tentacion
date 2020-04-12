use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod style;

fn main() {
    let text = dom::text("Tentacion".to_string());
    let elem = dom::elem("div".to_string(), HashMap::new(), vec![text]);

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

    let stylesheet = html::parse(css_source);
    println!("Stylesheet: {:?}: ", stylesheet);
}
