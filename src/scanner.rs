use std::collections::HashMap;
use crate::scanner::Token::*;
use crate::scanner::Error::*;
use crate::scanner::Associativity::*;

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
    keywords: HashMap<String,Token>,
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
    pub fn new(input: String, keywords: HashMap<String,Token>) -> Scanner {
        Scanner {
            input,
            start: 0,
            position: 0,
            current_char: Option::None,
            line: 1,
            error:Option::None,
            column: 1,
            tokens: Vec::new(),
            keywords,
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

        let token = Token::new(&self.input[self.start..self.position],&self.keywords);
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
                '>' => {
                    if_peek_match_scan!(self, '=');
                    self.scan_token()
                },
                '<' => {
                    if_peek_match_scan!(self, '=');
                    self.scan_token()
                }
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
    OpenDelimiter(char),
    CloseDelimiter(char),
    Equal,
    Comma,
    Semicolon,
    Colon,
    Exclamation,

    //Boolean Operators
    NotEq,Eq, LessThan,GreaterThan,LessThanEq,GreaterThanEq,

    //Keywords
    Fn,Let,Axiom,If,While,For,Var,
}
pub enum Associativity{
    Left,Right,None
}

impl Token{
    pub fn new(input: &str,keywords:&HashMap<String, Token>) -> Result<Token,Error>{
        match input{
            a @ ("+" |  "-" | "*" | "/" | "^")  => Ok(Operator(a.chars().next().unwrap())),
            c @ ("(" |  "{")  => Ok(OpenDelimiter(c.chars().next().unwrap())),
            c @ (")" | "}")  => Ok(CloseDelimiter(c.chars().next().unwrap())),
            "," => Ok(Comma),
            ";" => Ok(Semicolon),
            ":" => Ok(Colon),
            "=" => Ok(Equal),
            "!" => Ok(Exclamation),

            "!=" => Ok(NotEq),
            "==" => Ok(Eq),
            "<" => Ok(LessThan),
            ">" => Ok(GreaterThan),
            "<=" => Ok(LessThanEq),
            ">=" => Ok(GreaterThanEq),

            x if keywords.contains_key(x) => Ok(keywords[x].clone()),

            x if x.chars().all(|c| c.is_numeric() || c == '.') => match x.parse::<i32>() {
                Ok(x) => Ok(Number(x)),
                Err(_) => Err(InvalidToken(x.to_string())),
            },
            x if x.chars().next().unwrap().is_alphabetic() =>{
                Ok(Identifier(x.to_string()))
            },
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

    pub fn associativity(&self) -> Associativity{
        match self{
            Operator(a) =>{
                match a{
                    '+' => Left,
                    '-' => Left,
                    '*' => Left,
                    '/' => Left,
                    '^' => Right,
                    _=> None,
                }

            },
            Equal         |
            Eq            |
            NotEq         |
            GreaterThan   |
            LessThan      |
            GreaterThanEq |
            LessThanEq => Right,


            _=> None,
        }
    }
}





macro_rules! token_keywords_map {
    ($($str:ident),*) => {
        {
            let mut m:HashMap<String, Token> = HashMap::new();
            $(
            m.insert(stringify!($str).to_lowercase(),Token::$str);
            )*
            m
        }
    };
}
pub fn tokenize (input: String) -> Vec<Token> {

    let keywords:HashMap<String,Token> = token_keywords_map![Fn,Let,Axiom,If,While,For,Var];

    let mut scanner = Scanner::new(input,keywords);
    scanner.scan();
    scanner.tokens
}
