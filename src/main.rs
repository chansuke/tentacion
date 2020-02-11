use std::collections::HashMap;

pub mod dom;

fn main() {
    let text = dom::text("Tentacion".to_string());
    let elem = dom::elem("div".to_string(), HashMap::new(), vec![text]);
    println!("Trying to print out {:?}", elem);
}
