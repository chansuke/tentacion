use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub mod dom;
pub mod html;

fn main() {
    let text = dom::text("Tentacion".to_string());
    let elem = dom::elem("div".to_string(), HashMap::new(), vec![text]);
    println!("Trying to print out {:?}", elem);

    let mut html_source = "".to_string();
    let mut options = OpenOptions::new();
    let mut file = options.read(true).open("./examples/test.html").unwrap();
    file.read_to_string(&mut html_source)
        .ok()
        .expect("cannot read file");
    let root_node = html::parse(html_source);
    println!("{:?}", root_node);
}
