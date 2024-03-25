use std::fmt;



pub enum Token{
    Identifier(String,u16,u16),
    Number(String,u16,u16),
    Invalid(String,u16,u16),
    Plus(u16,u16),
    Minus(u16,u16),
    Mult(u16,u16),
    Div(u16,u16),
    Pow(u16,u16),
    OpenPar(u16,u16),
    ClosePar(u16,u16),
    Assign(u16,u16),
}
impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
           Token::Identifier(x,col,line) =>{let x = format!("{:?}",x);write!(f,"Identifier:|{}| at ({},{})",x,col,line)}, 
           Token::Number(x,col,line) =>{let x = format!("{:?}",x);write!(f,"Identifier:|{}| at ({},{})",x,col,line)}, 
           Token::Invalid(x,col,line) =>       {let x = format!("{:?}",x);write!(f, "|{}| at ({},{})",x,col,line)}, 
           Token::Plus(col,line) =>           write!(f, "|+| at ({},{})",col,line), 
           Token::Minus(col,line)=>           write!(f, "|-| at ({},{})",col,line), 
           Token::Mult(col,line) =>           write!(f, "|*| at ({},{})",col,line), 
           Token::Div(col,line)  =>           write!(f, "|/| at ({},{})",col,line), 
           Token::Pow(col,line)  =>           write!(f, "|^| at ({},{})",col,line), 
           Token::OpenPar(col,line) =>           write!(f, "|(| at ({},{})",col,line), 
           Token::ClosePar(col,line)  =>           write!(f, "|)| at ({},{})",col,line), 
           Token::Assign(col,line) =>           write!(f, "|:=| at ({},{})",col,line), 

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
    let mut col:u16 = 0;
    let mut token = String::new();
    let iter = input.chars();
        for(i,c) in iter.enumerate(){ 
                if token.is_empty(){
            col = i as u16;
        };
                match c{
                    ' '  => {if token.is_empty(){continue} else {output.push(evaluate(token.clone(),col,line));
                            token.clear()}
                        },
                    '1'..='9' | 'a'..='z' | 'A'..='Z' | '_'| ':'=> {
                                                
                                                if input.len() - 1 == i {
                                                token.push(c);
                                                output.push(evaluate(token.clone(),col,line));
                                                } 
                                                
                                                else {token.push(c)};},
                    '^'|'+' | '-' | '*' | '%' |'['..=']'|'{'..='}'|'&'..='/'|';'..='?'|'\n' =>{
                        if token.is_empty(){
                        
                        token = String::from(c);
                        output.push(evaluate(token.clone(),col,line));
                        token.clear();
                        }
                        else if token.chars().nth(0).unwrap() == ':'{
                        token.push(c);
                        output.push(evaluate(token.clone(),col,line));
                        token.clear();
                            }
                        else {  
                        output.push(evaluate(token.clone(),col,line));
                        col= i as u16;
                        token = String::from(c);
                        output.push(evaluate(token.clone(),col,line));
                        token.clear();
                        }
                        
                        
                    },
                    
                       

                     _ => {output.push(evaluate(token.clone(),col,line));
                           token.push(c);
                           output.push(evaluate(token.clone(),col,line));
                           token.clear();}
                    }
                };
    output.into_iter().filter(|x| match x{
        Token::Invalid(x,_,_) => if x != ""{true} else {false},
        Token::Identifier(x,_,_) => if x != ""{true} else {false},
        _=> true,
    } ).collect()
    }
fn evaluate(input:String,col:u16,line:u16) -> Token{
    match input.len(){
        1 =>match input.chars().nth(0).unwrap(){ 
            '1'..='9' => Token::Number(input,col,line), 
            'a'..='z' => Token::Identifier(input,col,line),
            '+' => Token::Plus(col,line),
            '-' => Token::Minus(col,line),
            '*' => Token::Mult(col,line),
            '/' => Token::Div(col,line),
            '(' => Token::OpenPar(col,line),
            ')' => Token::ClosePar(col,line),
            '^' => Token::Pow(col,line),
            _=> Token::Invalid(input,col,line)},
        _ =>match input.chars().nth(0).unwrap_or_else(|| {println!("invalid Token {} at {},{}",input,col,line); std::process::exit(1)}){ 
            '1'..='9' => Token::Number(input,col,line), 
            'a'..='z' => Token::Identifier(input,col,line),
            ':' => match input.chars().nth(1){
                None => Token::Invalid(input,col,line),
                Some(a) => match a{
                '=' => Token::Assign(col,line),
                _=> Token::Invalid(input,col,line),

                }
            }
        _=> Token::Identifier(input,col,line),}
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

