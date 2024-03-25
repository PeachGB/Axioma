use std::collections::HashMap;
use crate::scanner::Token;

pub fn simulate_program(input:Vec<Token>){
    let mut stack:Vec<String> = vec![];
    let mut memory:HashMap<String, String> = HashMap::new();
    for token in input{
        match token{
            Token::Number(x,_,_) => {
                stack.push(x)},
            Token::Identifier(x,_,_) => {
                let value = memory.get(&x);
                match value{
                Some(v) => stack.push(v.clone()),
                None => stack.push(x),
                }},
            Token::Plus(_,_) => {
                let input1 = variable(stack.pop().unwrap(),&memory);
                let input2 = variable(stack.pop().unwrap(),&memory);
                stack.push(op_plus(input1,input2))},
            Token::Minus(_,_) => {
                let input1 = variable(stack.pop().unwrap(),&memory);
                stack.push(op_minus(input1))},
            Token::Mult(_,_) => {
                let input1 = variable(stack.pop().unwrap(),&memory);
                let input2 = variable(stack.pop().unwrap(),&memory);
                stack.push(op_mult(input1,input2))},
            Token::Div(_,_) => {
                let input1 = variable(stack.pop().unwrap(),&memory);
                let input2 = variable(stack.pop().unwrap(),&memory);
                stack.push(op_div(input1,input2))},
            Token::Pow(_,_) =>{
                let input1 = variable(stack.pop().unwrap(),&memory);
                let input2 = variable(stack.pop().unwrap(),&memory);
                stack.push(op_pow(input1,input2))
            },
            Token::Assign(_,_) =>{
                let value = stack.pop().unwrap();
                let variable = stack.pop().unwrap();
                memory.insert(variable,value);
                },
            
            Token::Invalid(i,row,line) =>{
                println!("ERROR: Invalid Token {} at row:{},line:{}",i,row,line);
                std::process::exit(1);
            }
            _=> {println!("UNREACHEABLE");
            std::process::exit(1);},
        }
    
    } 

}

fn variable(input:String,memory:&HashMap<String,String>) -> String{
    if ('1'..='9').contains(&input.chars().nth(0).unwrap()){
        input
    } else {
    let output = memory.get(&input);
        match output{
        Some(v) => v.clone(),
        None => {println!("ERROR: undefined variable: |{}|", input);
            std::process::exit(1);},
                
        }
    }
}

fn op_plus(x:String,y:String)-> String{
   let out:i32 = x.parse::<i32>().unwrap() + y.parse::<i32>().unwrap();
    out.to_string()
}
fn op_minus(x:String)-> String{
   let out:i32 = -x.parse::<i32>().unwrap();
    out.to_string()
}
fn op_mult(x:String,y:String)-> String{
let out:i32 = x.parse::<i32>().unwrap()*y.parse::<i32>().unwrap();
    out.to_string()
}
fn op_div(x:String,y:String)-> String{
let out:i32 = y.parse::<i32>().unwrap()/x.parse::<i32>().unwrap();
    out.to_string()
}
fn op_pow(x:String,y:String) -> String{
    let out1:i32 = x.parse::<i32>().unwrap();
    let out2:i32 = y.parse::<i32>().unwrap();
    let out = out1.pow(out2 as u32);
    out.to_string()
}
