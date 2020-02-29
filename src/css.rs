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

fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector { tag_name: None, id: None, class: Vec::new() };
    while !self.eof() {
        match self.next_char() {
            '#' => {
                self.consume_char();
                selector.id = Some(self.parse_identifier());
            }
            '.' => {
                self.consume_char();
                selector.class.push(self.parse_identifier());
            }
            '*' => {
                self.consume_char();
            }
            c if valid_identifier_char(c) => {
                selector.tag_name = Some(self.parse_identifier());
            }
            _ => break
        }
    }
    return selector;
}
