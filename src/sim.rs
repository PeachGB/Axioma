use std::collections::HashMap;
use crate::scanner::Token::{*,self};
use crate::parser;
pub fn simulate_program(input:Vec<Token>) -> (Vec<String>,HashMap<String,String>){
    // for t in &input{
    //     print!("{},",t)
    // };
    let mut stack:Vec<String> = vec![];
    let mut expression_stack:Vec<Token> = vec![];
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
            Token::Expression(_,_,_) => expression_stack.push(token.clone()),
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
            Token::Greater(_,_) =>{
                let value2 = stack.pop().unwrap();
                let value1 = stack.pop().unwrap();
                stack.push(op_greater(value1,value2))
            }
            Token::Less(_,_) =>{
                let value2 = stack.pop().unwrap();
                let value1 = stack.pop().unwrap();
                stack.push(op_less(value1,value2))
            }
            Token::Equal(_,_) =>{
                let value2 = stack.pop().unwrap();
                let value1 = stack.pop().unwrap();
                stack.push(op_equal(value1,value2))
            }
            Token::If(_,_) =>{
                let value = stack.pop().unwrap();
                let exp = expression_stack.pop().unwrap();
                if let Some(mem) = op_if(value,exp,&memory){
                    for (key,value) in mem.iter(){
                        memory.insert(key.to_string(),value.to_string());
                    }
                }

                
            }

            
            Token::Print(_,_) => {
                let value = variable(stack.pop().unwrap(),&memory);
                println!("{}",value)}, 
            Token::Invalid(i,row,line) =>{
                println!("ERROR: Invalid Token {} at row:{},line:{}",i,row,line);
                std::process::exit(1);
            }
            x=> {println!("Token not Defined: |{}|",x);
            std::process::exit(1);},
        }

    
    } 
   return (stack,memory) 

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
    let out2:i32 = x.parse::<i32>().unwrap();
    let out1:i32 = y.parse::<i32>().unwrap();
    let out = out1.pow(out2 as u32);
    out.to_string()
}
fn op_greater(x:String,y:String) -> String{
    if x.parse::<i32>().unwrap() > y.parse::<i32>().unwrap(){
    "true".to_string()
} else {
    "false".to_string()
    }
}
fn op_less(x:String,y:String) -> String{
    if x.parse::<i32>().unwrap() < y.parse::<i32>().unwrap(){
    "true".to_string()
} else {
    "false".to_string()
    }
}
fn op_equal(x:String,y:String) -> String{
    if x.parse::<i32>().unwrap() == y.parse::<i32>().unwrap(){
    "true".to_string()
} else {
    "false".to_string()
    }
}
fn op_if(value:String,expression:Token,mem:&HashMap<String,String>)->Option<HashMap<String, String>> {
    if let Ok(boolean) = value.parse(){
        match boolean{
            true =>{
                let mut expression:Vec<Token> = Token::unwrap_Expression(expression);
                let parsed:Vec<Token> = parser::parse_line(expression);
                let (_,mem) = simulate_program(parsed);
                return Some(mem)

            },
            false =>return None,
        }
    }
    println!("expected boolean at");
    std::process::exit(1)

}

