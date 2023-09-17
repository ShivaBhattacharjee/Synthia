use std::collections::HashMap;
use std::string::String;

fn is_digit(ch: char) -> bool {
    ch as u8 >= '0' as u8 && ch as u8 <= '9' as u8
}

fn is_alpha(ch: char) -> bool {
    let uch = ch as u8;
    (uch >= 'a' as u8 && uch <= 'z' as u8) || (uch >= 'A' as u8 && uch <= 'Z' as u8) || (ch == '_')
}

fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}

fn get_keywords_hashmap() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("and", And),
        ("class", Class),
        ("else", Else),
        ("false", False),
        ("for", For),
        ("fun", Fun),
        ("if", If),
        ("nil", Nil),
        ("or", Or),
        ("print", Print),
        ("return", Return),
        ("super", Super),
        ("this", This),
        ("true", True),
        ("var", Var),
        ("while", While),
    ])
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords_hashmap(),
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if errors.len() > 0 {
            let mut joined = "".to_string();
            for error in errors {
                joined.push_str(&error);
                joined.push_str("\n");
            }
            return Err(joined);
        }

        Ok(self.tokens.clone())
    }

    // var test = 0.01;

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token = if self.char_match('=') {
                    // !=
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.char_match('=') {
                    EqualEqual
                } else {
                    Equal
                };

                self.add_token(token);
            }
            '<' => {
                let token = if self.char_match('=') {
                    LessEqual
                } else if self.char_match('-') {
                    Gets
                } else {
                    Less
                };

                self.add_token(token);
            }
            '>' => {
                let token = if self.char_match('=') {
                    GreaterEqual
                } else {
                    Greater
                };

                self.add_token(token);
            }
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            },
            '|' => {
                if self.char_match('>') {
                    self.add_token(Pipe);
                } else {
                    return Err(format!("Expected '>' at line {}", self.line));
                }
            },
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,

            c => {
                if is_digit(c) {
                    self.number()?;
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(format!("Unrecognized char at line {}: {}", self.line, c));
                }
            }
        }

        Ok(())
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let substring = &self.source[self.start..self.current];
        if let Some(&t_type) = self.keywords.get(substring) {
            self.add_token(t_type);
        } else {
            self.add_token(Identifier);
        }
    }

    fn number(self: &mut Self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let substring = &self.source[self.start..self.current];
        let value = substring.parse::<f64>();
        match value {
            Ok(value) => self.add_token_lit(Number, Some(FValue(value))),
            Err(_) => return Err(format!("Could not parse number: {}", substring)),
        }

        Ok(())
    }

    fn peek_next(self: &Self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(self: &mut Self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token_lit(StringLit, Some(StringValue(value.to_string())));

        Ok(())
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn char_match(self: &mut Self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != ch {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        c
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.current].to_string();

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_number: self.line,
        });
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Single-char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One Or Two Chars
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Pipe, // |>
    Gets, // <-

    // Literals
    Identifier,
    StringLit,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
use TokenType::*;

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    FValue(f64),
    StringValue(String),
}
use LiteralValue::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
}

// impl Token {
//     pub fn to_string(self: &Self) -> String {
//         format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(( )) }{";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 7);
        assert_eq!(scanner.tokens[0].token_type, LeftParen);
        assert_eq!(scanner.tokens[1].token_type, LeftParen);
        assert_eq!(scanner.tokens[2].token_type, RightParen);
        assert_eq!(scanner.tokens[3].token_type, RightParen);
        assert_eq!(scanner.tokens[4].token_type, RightBrace);
        assert_eq!(scanner.tokens[5].token_type, LeftBrace);
        assert_eq!(scanner.tokens[6].token_type, Eof);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "! != == >=";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, Bang);
        assert_eq!(scanner.tokens[1].token_type, BangEqual);
        assert_eq!(scanner.tokens[2].token_type, EqualEqual);
        assert_eq!(scanner.tokens[3].token_type, GreaterEqual);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_string_lit() {
        let source = r#""ABC""#;
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, StringLit);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_string_lit_unterminated() {
        let source = r#""ABC"#;
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();
        match result {
            Err(_) => (),
            _ => panic!("Should have failed"),
        }
    }

    #[test]
    fn handle_string_lit_multiline() {
        let source = "\"ABC\ndef\"";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, StringLit);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC\ndef"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn number_literals() {
        let source = "123.123\n321.0\n5";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 4);
        for i in 0..3 {
            assert_eq!(scanner.tokens[i].token_type, Number);
        }
        match scanner.tokens[0].literal {
            Some(FValue(val)) => assert_eq!(val, 123.123),
            _ => panic!("Incorrect literal type"),
        }
        match scanner.tokens[1].literal {
            Some(FValue(val)) => assert_eq!(val, 321.0),
            _ => panic!("Incorrect literal type"),
        }
        match scanner.tokens[2].literal {
            Some(FValue(val)) => assert_eq!(val, 5.0),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn get_identifer() {
        let source = "this_is_a_var = 12;";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 5);

        assert_eq!(scanner.tokens[0].token_type, Identifier);
        assert_eq!(scanner.tokens[1].token_type, Equal);
        assert_eq!(scanner.tokens[2].token_type, Number);
        assert_eq!(scanner.tokens[3].token_type, Semicolon);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn get_keywords() {
        let source = "var this_is_a_var = 12;\nwhile true { print 3 };";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 13);

        assert_eq!(scanner.tokens[0].token_type, Var);
        assert_eq!(scanner.tokens[1].token_type, Identifier);
        assert_eq!(scanner.tokens[2].token_type, Equal);
        assert_eq!(scanner.tokens[3].token_type, Number);
        assert_eq!(scanner.tokens[4].token_type, Semicolon);
        assert_eq!(scanner.tokens[5].token_type, While);
        assert_eq!(scanner.tokens[6].token_type, True);
        assert_eq!(scanner.tokens[7].token_type, LeftBrace);
        assert_eq!(scanner.tokens[8].token_type, Print);
        assert_eq!(scanner.tokens[9].token_type, Number);
        assert_eq!(scanner.tokens[10].token_type, RightBrace);
        assert_eq!(scanner.tokens[11].token_type, Semicolon);
        assert_eq!(scanner.tokens[12].token_type, Eof);
    }

    #[test]
    fn gets_keyword() {
        let source = "fun cmd <- \"echo hello\";";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 6);

        assert_eq!(scanner.tokens[0].token_type, Fun);
        assert_eq!(scanner.tokens[1].token_type, Identifier);
        assert_eq!(scanner.tokens[2].token_type, Gets);
        assert_eq!(scanner.tokens[3].token_type, StringLit);
        assert_eq!(scanner.tokens[4].token_type, Semicolon);
        assert_eq!(scanner.tokens[5].token_type, Eof);
    }
}
