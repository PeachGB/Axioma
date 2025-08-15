use std::collections::HashMap;
use crate::scanner::{Token, Token::*};

pub struct Scope{
    Scope:Vec<Token>,
    Memory:HashMap<String, Token>,
}

pub enum Expression{
    Arithmetic(Vec<Token>),
    Logic(Vec<Token>),
}
pub enum Statement{
    Assignment(String, Expression),
    If(Expression, Vec<Statement>),
    While(Expression, Vec<Statement>),
    Call(String, Vec<Expression>),
    Print(Expression),
    
}




pub fn process(tokens: Vec<Token>) -> Vec<Token> {
    let mut tokens = tokens.clone();
    let output:Vec<Token> = vec![];
    check_invalid(tokens);
    output
}