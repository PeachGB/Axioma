use std::{
    env,
    fs};
mod scanner;

fn main(){
    //read file from args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{panic!("USAGE: teorema [FILEPATH] [FLAGS](FLAGS NOT IMPLEMENTED) \nERROR Incorrect Usage");}
    let input = match fs::read_to_string(args[1].clone()){
        Ok(file) => file,
        Err(error) => panic!("ERROR: {:?} could not open the file",error)
    };
    //Takes a string and returns Vec<Token>
    let lexer = scanner::tokenizer(input);
    
    for token in lexer.iter(){
        for tok in token.iter(){
        print!("{}",tok);}
    }

}
