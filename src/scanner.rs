use std::fmt;


pub enum Token{
    Identifier(String,u16,u16),
    Symbol(String,u16,u16),
    Plus(u16,u16),
    Minus(u16,u16),
    Mult(u16,u16),
    Div(u16,u16),
}
impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
           Token::Identifier(x,col,line) =>{let x = format!("{:?}",x);write!(f,"Identifier:|{}| at ({},{})",x,col,line)}, 
           Token::Symbol(x,col,line) =>       {let x = format!("{:?}",x);write!(f, "|{}| at ({},{})",x,col,line)}, 
           Token::Plus(col,line) =>           write!(f, "|+| at ({},{})",col,line), 
           Token::Minus(col,line)=>           write!(f, "|-| at ({},{})",col,line), 
           Token::Mult(col,line) =>           write!(f, "|*| at ({},{})",col,line), 
           Token::Div(col,line)  =>           write!(f, "|/| at ({},{})",col,line), 

        }
    }
}

           
fn lines(input:String) -> Vec<String>{
    let mut output: Vec<String> = vec![];
    for line in input.lines(){
        output.push(line.to_string());
    }
    output

}
fn scan(input:String,line:u16) -> Vec<Token>{
    let mut output:Vec <Token> = vec![];
    let mut row:u16 = 0;
    let mut token = String::new();
    let iter = input.chars();
        for(i,c) in iter.enumerate(){ 
                if token.is_empty(){
            row = i as u16;
        };
                match c{
                    ' '  => {output.push(evaluate(token.clone(),row,line));
                            token.clear()
                        },
                    '1'..='9' | 'a'..='z' | 'A'..='Z' | '_'=> {
                                                
                                                if input.len() - 1 == i {
                                                token.push(c);
                                                output.push(evaluate(token.clone(),row,line));
                                                } 
                                                
                                                else {token.push(c)};},
                    '+' | '-' | '*' | '%' |'['..=']'|'{'..='}'|'&'..='/'|':'..='?'|'\n' =>{
                        if token.is_empty(){
                        
                        token = String::from(c);
                        output.push(evaluate(token.clone(),row,line));
                        token.clear();
                        }
                        else {  
                        output.push(evaluate(token.clone(),row,line));
                        row= i as u16;
                        token = String::from(c);
                        output.push(evaluate(token.clone(),row,line));
                        token.clear();
                        }
                        
                        
                    },

                     _ => {output.push(evaluate(token.clone(),row,line));
                           token.clear();}
                    }
                };
    output.into_iter().filter(|x| match x{
        Token::Symbol(x,_,_) => if x != ""{true} else {false},
        Token::Identifier(x,_,_) => if x != ""{true} else {false},
        _=> true,
    } ).collect()
    }
fn evaluate(input:String,row:u16,line:u16) -> Token{
    match input.len(){
        1 =>match input.chars().nth(0).unwrap(){ 
            '1'..='9' => Token::Identifier(input,row,line), 
            '+' => Token::Plus(row,line),
            '-' => Token::Minus(row,line),
            '*' => Token::Mult(row,line),
            '/' => Token::Div(row,line),
            _=> Token::Symbol(input,row,line)},
        _ => Token::Identifier(input,row,line),
    }
} 



pub fn tokenizer(input:String) ->Vec<Vec<Token>>{
    let mut output:Vec<Vec<Token>> = vec![];
    let lines = lines(input.clone());
    let mut line:u16 = 0;
    for l in lines{
    output.push(scan(l,line));
        line += 1
    }
    output
}

