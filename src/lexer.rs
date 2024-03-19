pub enum Keywords{
    Pass,
}

pub enum Separators{
    OpenPar,
    ClosePar,
    OpenBra,
    CloseBra,
    
}

pub enum Operators{
    Plus,
    Minus,
    Mult,
    FlDiv,
    IntDivg,
    Moduleg,
    Equal,
    
}
pub enum IntegerType{
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}
pub enum Literals{
    Bool(bool),
    Integer(IntegerType),
    Floats(String),
    Char(String),
    String(String),
}
pub enum Token{
    Identifier(String),
    Keyword(Keywords),
    Separator(Separators),
    Operator(Operators),
    Literal(Literals),
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
    
pub fn evaluator(Vec<Strings>)-> Vec<Token>{
    todo!();
}




