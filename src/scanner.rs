use std::fmt;

#[derive(Clone,Debug)]
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
    Greater(u16,u16),
    Less(u16,u16),
    If(u16,u16),
    Equal(u16,u16),
    Print(u16,u16),
    NewLine,
    Expression(Vec<Token>,u16,u16),
}
impl Token{
    pub fn unwrap_Expression(input:Token) -> Vec<Token>{
        match input{
            Token::Expression(exp,_,_) => {let output:Vec<Token> = exp;
                                            output},
            x=> {println!("Must input an Expression not {}",x);
            std::process::exit(1)}
        }
    }
}
impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Token::Identifier(x,_,_) =>{let x = format!("{:?}",x);write!(f,"|{}|",x)}, 
            Token::Number(x,_,_) =>{let x = format!("{:?}",x);write!(f,"|{}|",x)}, 
            Token::Invalid(x,_,_) =>       {let x = format!("{:?}",x);write!(f, "|{}|",x)}, 
            Token::Plus(_,_) =>           write!(f, "|+|"), 
            Token::Minus(_,_)=>           write!(f, "|-|"), 
            Token::Mult(_,_) =>           write!(f, "|*|"), 
            Token::Div(_,_)  =>           write!(f, "|/|"), 
            Token::Pow(_,_)  =>           write!(f, "|^|"), 
            Token::OpenPar(_,_) =>             write!(f, "|(|"), 
            Token::ClosePar(_,_)  =>           write!(f, "|)|"), 
            Token::Assign(_,_) =>              write!(f, "|:=|"), 
            Token::Greater(_,_) =>             write!(f, "|>|"), 
            Token::Less(_,_) =>                write!(f, "|<|"), 
            Token::If(_,_) =>                  write!(f, "|->|"),              
            Token::Equal(_,_) =>               write!(f, "|=|"),
            Token::Print(_,_) =>               write!(f, "|$|"),
            Token::NewLine => write!(f, "EndOfLine"),
            Token::Expression(_,_,_) =>        write!(f,"Expression"), 

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
                    '1'..='9' | 'a'..='z' | 'A'..='Z' | '_'| ':'|'-'=> {
                                                
                                                if input.len() - 1 == i {
                                                token.push(c);
                                                output.push(evaluate(token.clone(),col,line));
                                                } 
                                                
                                                else {token.push(c)};},
                    '<'|'>'|'^'|'+' | '*' | '/' |'!'|'('|')'|'='|'$' =>{                      
                        if token.is_empty(){                                                                                    
                                                                                                                                
                        token = String::from(c);                                                                                
                        output.push(evaluate(token.clone(),col,line));                                                          
                        token.clear();                                                                                          
                        }                                                                                                       
                        else if token.chars().nth(0).unwrap() == ':' || token.chars().nth(0).unwrap() == '-'{                   
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
            '>' => Token::Greater(col,line),
            '<' => Token::Less(col,line),
            '=' => Token::Equal(col,line),
            '$' => Token::Print(col,line),
            _=> Token::Invalid(input,col,line)},
        _ =>match input.chars().nth(0).unwrap_or_else(|| {println!("invalid Token {} at {},{}",input,col,line); std::process::exit(1)}){ 
            '1'..='9' => Token::Number(input,col,line), 
            'a'..='z' => Token::Identifier(input,col,line),
            '-' => match input.chars().nth(1){
                None => Token::Invalid(input,col,line),
                Some(a) =>match a{
                    '>' => Token::If(col,line),
                    _=> Token::Invalid(input,col,line),
                }
            }
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

fn reverse_vec<T>(input:&Vec<T>) -> Vec<T> where T:Clone{
    let mut stack:Vec<T> = input.clone();
    let mut output:Vec<T> = vec![];
    while let Some(t) = stack.pop(){
        output.push(t);
    }
    output
}

fn rescan(input:Vec<Token>)-> Vec<Token>{
    let mut output:Vec<Token> = input.clone();
    let mut expression:Vec<Token> = vec![];
    while let Some(t) = output.pop(){
        match t{
            Token::If(col,line) => {output.push(t);
                output.push(Token::Expression(reverse_vec(&expression),col+2,line));
                expression.clear();
                return output
            },
            _=> expression.push(t),
        }
    }
while let Some(t) = expression.pop(){
        output.push(t);
    }
output
}
pub fn tokenizer(input:String) ->Vec<Vec<Token>>{
    let mut output:Vec<Vec<Token>> = vec![];
    let lines = lines(input.clone());
    let mut line:u16 = 0;
    for l in lines{
    let mut inp:Vec<Token> = scan(l,line);
    inp.push(Token::NewLine);
    let inp:Vec<Token> = rescan(inp);
    output.push(inp);
        line += 1
    }
    output
}

