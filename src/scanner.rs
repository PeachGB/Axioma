use crate::scanner::Token::*;
use crate::scanner::Error::*;


pub enum Error {
    InvalidCharacter( char),
    InvalidToken(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            InvalidCharacter(x) => write!(f,"Invalid character: {}",x),
            InvalidToken(x) => write!(f,"Invalid token: {}",x),
        }
    }
}
pub struct Scanner {
    input: String,
    start: usize,
    position: usize,
    current_char: Option<char>,
    line: usize,
    column: usize,
    error: Option<Error>,
    tokens: Vec<Token>,
}

macro_rules! if_peek_match_scan {
        ($self:ident, $($input:expr),*) => {
            {
                $(if $self.peek_match($input){
                    $self.advance();
                    $self.scan_token();
                    continue
                })*
            }
        };
    }
impl Scanner {
    pub fn new(input: String) -> Scanner {
        Scanner {
            input,
            start: 0,
            position: 0,
            current_char: None,
            line: 1,
            error:None,
            column: 1,
            tokens: Vec::new(),
        }
    }
    fn peek(&self) -> Option<char>
    {
        self.input.chars().nth(self.position)
    }
    fn peek_match(&self, input: char) -> bool {
        if let Some(x) = self.peek() {
            if x == input {
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Option<char> {
        self.current_char = self.input.chars().nth(self.position);
        self.position += 1;
        if let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }

        self.current_char
    }


    fn scan_token(&mut self) {

        let token = Token::new(&self.input[self.start..self.position]);
        match token{
            Ok(x) => self.tokens.push(x),
            Err(x) => self.error = Some(x),
        }
        if let Some(x) = &self.error{
            panic!("Error: {} at line {}, and column {}",x,self.line,self.column);
        }
       while let Some(c) = self.peek() {
           if c.is_whitespace() {
               self.advance();
               continue;
           }
           else {
               self.start = self.position;
               break;
           }
       }

    }
    pub fn scan(&mut self) {
        while let Some(c) = self.advance(){
            match c{
                ' ' | '\t' | '\n' | '\r' => continue,
                '(' => self.scan_token(),
                ')' => self.scan_token(),
                '{' => self.scan_token(),
                '}' => self.scan_token(),
                ',' => self.scan_token(),
                ';' => self.scan_token(),
                ':' => self.scan_token(),
                '+' => self.scan_token(),
                '-' => self.scan_token(),
                '!' => {
                    if_peek_match_scan!(self, '=');
                    self.scan_token()
                }
                '=' => {
                    if_peek_match_scan!(self, '=');
                    self.scan_token()
                }

                x if x.is_numeric() => {
                    while let Some(c) = self.peek(){
                        if !c.is_numeric() &&  c != '.' {
                            self.scan_token();
                            break;
                        }
                        self.advance();
                    }
                },
                x if x.is_alphabetic() => {
                    while let Some(c) = self.peek(){
                        if !c.is_alphabetic() && !c.is_numeric() && c != '_' && c != '.'  {
                            self.scan_token();
                            break;
                        }
                        self.advance();
                    }
                }
                x => {self.error = Some(InvalidCharacter(x));

                    if let Some(x) = &self.error{
                        panic!("Error: {} at line {}, and column {}",x,self.line,self.column);
                    }
                },
            }

        }

    }


}

#[derive(Debug,Clone,Eq, PartialEq)]
pub enum Token {
    //Literal Token
    Number(i32),
    Identifier(String),
    //Single Char Token
    Operator(char),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Equal,
    Comma,
    Semicolon,
    Colon,
    Exclamation,

    //Boolean Operators
    NotEq,
}

impl Token{
    pub fn new(input: &str) -> Result<Token,Error>{
        match input{
             a @ ("+" |  "-" | "*" | "/" | "^")  => Ok(Operator(a.chars().next().unwrap())),
            "(" => Ok(LeftParen),
            ")" => Ok(RightParen),
            "{" => Ok(LeftBrace),
            "}" => Ok(RightBrace),
            "," => Ok(Comma),
            ";" => Ok(Semicolon),
            ":" => Ok(Colon),
            "=" => Ok(Equal),
            "!" => Ok(Exclamation),
            "!=" => Ok(NotEq),
            x if x.chars().all(char::is_numeric) => match x.parse::<i32>() {
                Ok(x) => Ok(Number(x)),
                Err(_) => Err(InvalidToken(x.to_string())),
            },
            x if x.chars().next().unwrap().is_alphabetic() => Ok(Identifier(x.to_string())),
            x => Err(InvalidToken(x.to_string())),
        }
    }
        pub fn len(&self)-> usize{
            match self{
                Number(x) => x.to_string().len(),
                Identifier(x) => x.len(),
                _ => 1
            }

        }

}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Number(x) => write!(f,"Number: {}",x),
            Identifier(x) => write!(f,"Identifier: {}",x),
            Operator(x) => write!(f,"Operator: {}",x),
            Equal => write!(f,"Equal"),
            LeftParen => write!(f,"Left Paren"),
            RightParen => write!(f,"Right Paren"),
            LeftBrace => write!(f,"Left Brace"),
            RightBrace => write!(f,"Right Brace"),
            Comma => write!(f,"Comma"),
            Semicolon => write!(f,"Semicolon"),
            Colon => write!(f,"Colon"),
            Exclamation => write!(f,"Exclamation"),
            NotEq => write!(f,"Not Equal"),
        }
    }
}



pub fn tokenize (input: String) -> Vec<Token> {
    let mut scanner = Scanner::new(input);
    scanner.scan();
    scanner.tokens
}