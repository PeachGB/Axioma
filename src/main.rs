use std::{
env,
    fs};
mod scanner;
mod parser;
mod sim;
fn main(){
    //read file from args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{panic!("USAGE: teorema [FILEPATH] [FLAGS](FLAGS NOT IMPLEMENTED) \nERROR Incorrect Usage");}
    let input = match fs::read_to_string(args[1].clone()){
        Ok(file) => file,
        Err(error) => panic!("ERROR: {:?} could not open the file",error)
    };
    //Takes a string and returns Vec<Vec<Token>>
    let lexer = scanner::tokenizer(input);
    //takes Vec<Vec<Token>> and returns a Vec<Token> in order of operations
    let program = parser::parse(lexer); 
    
    sim::simulate_program(program);

    

}
