use crate::scanner::Token;
use std::collections::HashMap;

enum Operation{
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    And,
    Or,
    Equal,
    Greater,
    Less,
    Number(i32),
    Identifier(String)
}
pub struct Interpreter {
    program: Vec<Token>,
    memory: HashMap<String, Token>,
    stack: Vec<Operation>,
    pc: usize,
}


impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            program: Vec::new(),
            memory: HashMap::new(),
            stack: Vec::new(),
            pc: 0,
        }
    }
    pub fn set(&mut self, program: Vec<Token>) {
        self.program = program;
    }
    pub fn get_program(&self) -> &Vec<Token> {
        &self.program
    }

    //+ 4 +
    // +
    //3 2
    fn eval(&mut self, t: Token) {
        match t {
            Token::Number(a) => self.stack.push(Operation::Number(a.parse().unwrap())),
            Token::Identifier(a) => self.stack.push(Operation::Identifier(a)),
            Token::Plus(a, b) => {
                self.stack.push(Operation::Plus);
                self.eval(*a);
                self.eval(*b);
            }
            Token::Minus(a, b) => {
                self.stack.push(Operation::Minus);
                self.eval(*a);
                self.eval(*b);
            }
            Token::Div(a, b) => {
                self.stack.push(Operation::Div);
                self.eval(*a);
                self.eval(*b);
            }
            Token::Mult(a, b) => {
                self.stack.push(Operation::Mult);
                self.eval(*a);
                self.eval(*b);
            }
            Token::Pow(a, b) => {
                self.stack.push(Operation::Pow);
                self.eval(*a);
                self.eval(*b);
            }
            Token::And(a, b) => {
                self.stack.push(Operation::And);
                self.eval(*a);
                self.eval(*b);
            }
            Token::Or(a, b) => {
                self.stack.push(Operation::Or);
                self.eval(*a);
                self.eval(*b);
            }
            Token::Equal(a, b) => {
                self.stack.push(Operation::Equal);
                self.eval(*a);
                self.eval(*b);
            }
            Token::Greater(a, b) => {
                self.stack.push(Operation::Greater);
                self.eval(*a);
                self.eval(*b);
            }
            Token::Less(a, b) => {
                self.stack.push(Operation::Less);
                self.eval(*a);
                self.eval(*b);
            }

            _ => panic!("Not implemented")
        }
    }
    pub fn go(&mut self) {
        while self.pc < self.program.len() {
            let mut token = &self.program[self.pc];
            match token {
                Token::Assign(a, b) => {
                    self.memory.insert(a.to_string(), *b.clone());

                    self.pc += 1
                },
                Token::Expression(expression) => {
                    self.eval(*expression.clone());
                    self.pc += 1
                }
                Token::If(a) =>{
                    self.eval(*a.clone());
                    
                    
                }
                tok => panic!("Not implemented")
            }
        }
let mut stack: Vec<i32> = Vec::new();
        while let Some(x) = self.stack.pop(){
            match x{
                Operation::Number(a) => stack.push(a),
                Operation::Identifier(a) => {
                    let expression = self.memory.get(&a).expect(format!("Identifier {} not found", a).as_str());
                    self.eval(expression.clone());
                }
                Operation::Plus => {
                    let a = stack.pop().expect("Not enough operands");
                    let b = stack.pop().expect("Not enough operands");
                    stack.push(a + b)
                }
                Operation::Minus => {
                    let a = stack.pop().expect("Not enough operands");
                    let b = stack.pop().expect("Not enough operands");
                    stack.push(a - b)
                }
                Operation::Mult => {
                    let a = stack.pop().expect("Not enough operands");
                    let b = stack.pop().expect("Not enough operands");
                    stack.push(a * b)
                }
                Operation::Div => {
                    let a = stack.pop().expect("Not enough operands");
                    let b = stack.pop().expect("Not enough operands");
                    stack.push(a / b)
                }
                Operation::Pow => {
                    let a = stack.pop().expect("Not enough operands");
                    let b = stack.pop().expect("Not enough operands");
                    stack.push(a.pow(b as u32))
                }
                Operation::And => {
                    let a = stack.pop().expect("Not enough operands");
                    let b = stack.pop().expect("Not enough operands");
                    stack.push(if a == 1 && b == 1 { 1 } else { 0 })
                }

            }
        }
    }
}
