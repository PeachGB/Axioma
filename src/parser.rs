use crate::scanner::Token;

fn precedence(op:&Token)-> u8{
    match op{
        Token::Plus(_,_) | Token::Minus(_,_) => 2,
        Token::Mult(_,_) | Token::Div(_,_) => 3,
        _=> 8,
    }
}

// expresion to parse
// 4+5+7*21
// 4 5+7 21*+
pub fn parse_expression(input:Vec<Vec<Token>>)->Vec<Token>{
let mut output:Vec<Token> = vec![];
let mut op_stack:Vec<Token> = vec![];
    for line in input{
        for token in line{
            match token{
                Token::Identifier(_,_,_) => output.push(token),
                _ => if op_stack.is_empty(){op_stack.push(token)} else {
    if precedence(&token) <= precedence(op_stack.last().unwrap()){
    output.push(op_stack.pop().unwrap());
        op_stack.push(token);
    } else{op_stack.push(token)}
}
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
