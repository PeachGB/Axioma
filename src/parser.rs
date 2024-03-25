use crate::scanner::Token;

fn precedence(op:&Token)-> u8{
    match op{
        Token::Plus(_,_) | Token::Minus(_,_) => 2,
        Token::Mult(_,_) | Token::Div(_,_) => 3,
        Token::Pow(_,_) => 4,
        Token::OpenPar(_,_) | Token::ClosePar(_,_) => 1,
        Token::Assign(_,_) => 0,
        _=> 8,
    }
}
fn parse_line(input:Vec<Token>)->Vec<Token>{
let mut output:Vec<Token> = vec![];
let mut op_stack:Vec<Token> = vec![];
    for token in input{
            match token{
                Token::Number(_,_,_) => output.push(token),
                Token::Identifier(_,_,_) => output.push(token),
                Token::OpenPar(_,_) => op_stack.push(token),
                Token::ClosePar(_,_) =>{
                while let Some(top_op) = op_stack.pop(){
                        match top_op{
                            Token::OpenPar(_,_) => break,
                            _=> output.push(top_op)
                        }
                    }
                }
                _ => if op_stack.is_empty(){op_stack.push(token)} else {
    if precedence(&token) <= precedence(op_stack.last().unwrap()){
    output.push(op_stack.pop().unwrap());
        op_stack.push(token);
    } else{op_stack.push(token)}
}
            }
        
    }  
    let mut i = op_stack.len();
while i != 0  {
    output.push(op_stack.pop().unwrap());
        i -= 1
    }
    output
}
pub fn parse(input:Vec<Vec<Token>>)-> Vec<Token>{
    let mut output:Vec<Token>=vec![];
    for line in input{
        output.extend(parse_line(line));
    }
    // for token in &output{
    //     println!("{}",token);
    // }
    output
}
