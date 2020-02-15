struct Parser {
    pos: usize, // An index of the character
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos].chars().next().unwrap();
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos].starts_with(s);
    }

    fn eof(&self) -> bool {
        self.pos == self.input.len();
    }

    fn consume_char(&mut self) -> char {
        let mut char = self.input.chars().nth(self.pos).unwrap();
        self.pos += next_pos;
        return char;
    }
}
