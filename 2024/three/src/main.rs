#[derive(Debug, PartialEq)]
enum Token {
    Number(u64),
    Identifier(String),
    Mul,
    Do,
    Dont,
    Whitespace,
    LParen,
    RParen,
    Comma,
    Unknown,
    EOF,
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let ch = match self.advance() {
            Some(c) => c,
            None => return Token::EOF,
        };

        match ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            c if c.is_ascii_alphabetic() => self.read_identifier(c),
            c if c.is_ascii_digit() => self.read_number(c),
            _ => Token::Unknown
        }
    }

    fn read_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first_char);

        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() || c != '_' {
                break;
            }
            identifier.push(c);
            self.advance();
        }

        match identifier.as_str() {
            "mul" => Token::Mul,
            "do" => Token::Do,
            "don't" => Token::Dont,
            _ => Token::Identifier(identifier),
        }
    }

    fn read_number(&mut self, first_char: char) -> Token {
        let mut num = String::new();
        num.push(first_char);
        
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() { break; }
            
            num.push(c);
            self.advance();
        };

        Token::Number(num.parse::<u64>().unwrap())
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() { break; };

            self.advance();
        };
    }
}

fn main() {
    let _bytes = lines_from_file("src/input.txt");
}

fn lines_from_file(filename: impl AsRef<std::path::Path>) -> std::io::Bytes<std::io::BufReader<std::fs::File>> {
    let file = std::fs::File::open(filename).expect("A file to open...");
    let buf = std::io::BufReader::new(file);
    std::io::Read::bytes(buf)
    // let line = std::io::BufRead::lines(buf).collect();
    // line
}
