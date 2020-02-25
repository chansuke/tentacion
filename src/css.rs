#[derive(Debug)]
pub struct StyleSheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

pub enum Selector {
    Simple(SimpleSelector),
}

pub struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

pub struct Declaration {
    name: String,
    value: Value,
}

pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

pub enum Unit {
    Px,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
