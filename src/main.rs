use std::collections::HashMap;
use std::fs::File;
use getopts::Options;

pub mod dom;
pub mod html;

fn main() {
    let text = dom::text("Tentacion".to_string());
    let elem = dom::elem("div".to_string(), HashMap::new(), vec![text]);
    println!("Trying to print out {:?}", elem);

    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");
    opts.optopt("o", "output", "Output file", "FILENAME");
    opts.optopt("f", "format", "Output file format", "png | pdf");
    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };
    let html = read_source(str_arg("h", "examples/test.html"));
    let root_node = html::parse(html)
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}
