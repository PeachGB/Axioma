use regex::Regex;

#[allow(dead_code)]
pub enum Keywords{
    Pass,
}
#[allow(dead_code)]
pub enum Arithmetic{
    Plus,
    Minus,
    Multiply,
    Division,
    Module,
}

#[allow(dead_code)]
pub enum Operators{
    Arithmetic(Arithmetic),

    Equal,
}

#[allow(dead_code)]
pub enum Separators{
    OpenPar,
    ClosePar,
    OpenBra,
    CloseSquareBrackets,
    OpenSquareBrackets
    
}

    

#[derive(Debug)]
pub enum Integer{
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}
#[derive(Debug)]
pub enum Float{
    F32(f32),
    F64(f64),
}
#[derive(Debug)]
pub enum Literals{
    Bool(bool),
    Integer(Integer),
    Floats(Float),
    Char(char),
    String(String),
}
#[derive(Debug)]
pub enum Token{
    Identifier(String),
    Keyword(String),
    Symbol(String),
    Literal(Literals),
    Invalid(String)
}



            

pub fn scan_token(input:String) -> Vec<String>{
    let mut output:Vec <String> = vec![];
    let mut token = String::new();
    let iter = input.chars();
        for(i,c) in iter.enumerate(){ 
                match c{
                    ' '  => {output.push(token.clone());
                            token.clear()
                        },
                    '1'..='9' | 'a'..='z' | 'A'..='Z' | '_'=> if input.len() - 1 == i {
                                                token.push(c);
                                                output.push(token.clone());} 
                                                
                                                else {token.push(c);},
                    '+' | '-' | '*' | '%' |'['..=']'|'{'..='}'|'&'..='/'|':'..='?'|'\n' =>{
                        if token.is_empty(){
                        token = String::from(c);
                        output.push(token.clone());
                        token.clear();}
                        else {    
                        output.push(token.clone());
                        token = String::from(c);
                        output.push(token.clone());
                        token.clear();}
                        
                        
                    },

                     _ => {output.push(token.clone());
                           token.clear();}
                    }
                };
    output.into_iter().filter(|x| x != "").collect()
    }
    
pub fn evaluator(input: Vec<String>)-> Vec<Token>{
    let output:Vec<Token> = input.into_iter().map(|x| {
    let alphanum_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    let num_regex = Regex::new(r"^[0-9]+$").unwrap();
        match x.len(){
            1 => Token::Symbol(x),
            _ => {if num_regex.is_match(&x){
                Token::Literal(Literals::Integer(Integer::U32(x.parse().unwrap())))
            }
                else if alphanum_regex.is_match(&x){
                    Token::Identifier(x)
                }
                else{
                    Token::Invalid(x)
                }
                } ,
        }
    }).collect();
        output
}




