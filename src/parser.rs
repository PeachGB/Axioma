use crate::scanner::{Token,Token::*};


pub struct Parser<'a>{
    token_stream: Vec<Token>,
    pos: usize,
    current: Option<&'a Token>,
    root: Ast,
}
pub struct Ast{
    root: Vec<AstNode>,
}
impl Ast{
    pub fn new() -> Self {
        Self {
            root: Vec::new(),
        }
    }
}
struct AstNode{
    expr: Box<dyn Expr>,
    branches: Vec<AstNode>,
}
impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token>) -> Self {
        Self {
            token_stream,
            current: None,
            pos:0,
            root: Ast::new(),
        }
    }
}
pub trait Expr{
}

pub struct Binary{
    operator: Token,
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}
pub struct Unary{
    operator: Token,
    right: Box<dyn Expr>,
}

pub struct Group{
    expression: Box<dyn Expr>,
}
pub struct Literal{
    value: Token,
}
pub struct call{
    identifier: String,
}

impl Expr for Binary{
}
impl Expr for Unary{}
impl Expr for Group{}
impl Expr for Literal{}



impl<'a> Parser<'a> {
    pub fn next(&mut self) -> Option<&Token> {
        self.current = self.token_stream.get(self.pos);
        self.pos += 1;
        self.current
    }
}
pub fn parse(token_stream: Vec<Token>){
    let mut parser = Parser::new(token_stream);



}
