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

    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let mut result = String::new();
        while　!self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(CharExt::is_whitespace);
    }
}
