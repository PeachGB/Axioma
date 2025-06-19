use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Expression(Box<Token>),
    Identifier(String),
    Number(String),
    //all binary operators have 2 pointers to his left and right side token
    Plus(Box<Token>, Box<Token>),
    Minus(Box<Token>, Box<Token>),
    Mult(Box<Token>, Box<Token>),
    Div(Box<Token>, Box<Token>),
    Pow(Box<Token>, Box<Token>),
    Assign(String,Box<Token>),
    Greater(Box<Token>, Box<Token>),
    Less(Box<Token>, Box<Token>),
    Equal(Box<Token>, Box<Token>),
    And(Box<Token>, Box<Token>),
    Or(Box<Token>, Box<Token>),
    Not(Box<Token>),
    Print(Box<Token>),
    //NAME maps Set to Set (pointers on uppercase)
    Map(Box<Token>, Vec<Token>, Vec<Token>),
    //Name, Set
    Set(Vec<Token>),
    //X: Conditions for x
    Comprehension(Box<Token>, Box<Token>),
    Invalid(String),
    If(Box<Token>),
    End,
    OpenPar,
    ClosePar,
    OpenBracket,
    CloseBracket,
    Lambda,
    NewLine,
    None,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(x) => {
                let x = format!("{:?}", x);
                write!(f, "|{}|", x)
            }
            Token::Number(x) => {
                let x = format!("{:?}", x);
                write!(f, "|{}|", x)
            }
            Token::Invalid(x) => {
                let x = format!("{:?}", x);
                write!(f, "|{}|", x)
            }
            Token::Plus(_, _) => write!(f, "|+|"),
            Token::Minus(_, _) => write!(f, "|-|"),
            Token::Mult(_, _) => write!(f, "|*|"),
            Token::Div(_, _) => write!(f, "|/|"),
            Token::Pow(_, _) => write!(f, "|^|"),
            Token::OpenPar => write!(f, "|(|"),
            Token::ClosePar=> write!(f, "|)|"),
            Token::Assign(_, _) => write!(f, "|:=|"),
            Token::Greater(_, _) => write!(f, "|>|"),
            Token::Less(_, _) => write!(f, "|<|"),
            Token::Map(func, a, b) => write!(f, "|{} : {:?}-> {:?}|", func, a, b),
            Token::If(_) => write!(f, "|if|"),
            Token::Equal(_, _) => write!(f, "|=|"),
            Token::Print(_) => write!(f, "|$|"),
            Token::NewLine => write!(f, "EndOfLine"),
            Token::Lambda => write!(f, "|λ|"),
            Token::Set(_) => write!(f, "|SET|"),
            Token::OpenBracket => write!(f, "|{{|"),
            Token::CloseBracket => write!(f, "|}}|"),
            Token::And(_, _) => write!(f, "|and|"),
            Token::Or(_, _) => write!(f, "|or|"),
            Token::Not(_) => write!(f, "|not|"),
            Token::Comprehension(_,_) => write!(f, "|comprehension|"),
            Token::Expression(_) => write!(f, "|expression|"),
            Token::End => write!(f, "|end|"),
            Token::None => write!(f, "None"),

        }
    }
}


static VALID_TOKENS: [char; 12+9+26] = ['1','2','3','4','5','6','7','8','9',
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    '+','-','*','/','(',')','^','>','<','=','\\', ' '];
static  DEFINER: [&str;4] = ["->","Let","if", "end"];



fn is_identifier(input: &str) -> bool {
    for x in input.chars() {
        match x{
            'a'..='z' | 'A'..='Z' | '_' | '-' => return true,
            _ => continue,
        }
    }
    false
}
fn parse_map(exp: Vec<&str>, line:u16 ) -> Token{
    if exp.len() == 0 {panic!("ERROR: UNREACHABLE {}", line) }
    let expression_name = exp[0];

    let mut map:bool = false;

    let mut exp:Vec<&str> = exp.to_vec();
    exp.remove(0);
    let mut codomain_stack:Vec<Token> = vec![];
    let mut domain_stack:Vec<Token> = vec![];
    while let Some(x) = exp.pop() {
        if x == "->" {
            map = true
        }
        if is_identifier(x){
                match map{
                    true => domain_stack.push(Token::Identifier(x.to_string())),

                    false =>codomain_stack.push(Token::Identifier(x.to_string())),
                }
            }




    }

Token::Map(Box::new(Token::Identifier(expression_name.to_string())),domain_stack,codomain_stack)


}

fn is_symbol(input:&str) -> bool{
    match input{
        "+" | "-" | "*" | "/" | "^" => true,
        _ => false

    }
}
fn is_number(input:&str) -> bool{
    match input{
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
        _ => false
    }
}
fn is_symbol_or_number(input:&str) -> bool{
    match input{
        "+" | "-" | "*" | "/" | "^" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
        _ => false

    }
}
fn is_bool_operator(input: &str) -> bool {
    match input {
        "or" | "and" | "not" => true,
        _ => false,
    }
}

fn precedence(input: &str) -> u8 {
    match input {
        "or" => 1,
        "and" => 2,
        ">" | "<" | "=" => 3,
        "+" | "-" | "not" => 4,
        "*" | "/" => 5,
        "^" => 6,
        _ => 0,
    }
}
fn shunting_yard(mut input: Vec<&str>) -> Vec<&str> {
    let mut output: Vec<&str> = vec![];
    let mut opstack: Vec<&str>= vec![];

    while let Some(x) = input.pop() {
        match x{
            "+" | "-" | "*" | "/" | "^" | ">" | "<" | "=" | "or" | "and" | "not" => {
                while let Some(y) = opstack.last() {
                    if precedence(x) <= precedence(y) {
                        output.push(y);
                        opstack.pop();
                    } else {
                        break;
                    }
                }
                opstack.push(x);
            }
            "(" => {
                opstack.push(x);
            }
            ")" => {
                while opstack.last() != Some(&"(") {
                    output.push(opstack.pop().expect("UNREACHABLE"));
                }
                output.push(opstack.pop().expect("UNREACHABLE"));
            }
            _=> {
                output.push(x);
            }
        }
    }
    while let Some(x) = opstack.pop() {
        output.push(x);
    }

output
}
fn parse_condition(input:Vec<&str>)-> Token{

    let mut stack: Vec<Token> = vec![];
    for token in input{
        match token{
            "and" =>{
                let left = stack.pop().expect("ERROR EVALUATING CONDITION");
                let right = stack.pop().expect("ERROR EVALUATING CONDITION");
                stack.push(Token::And(Box::new(left), Box::new(right)))
            }
            "or" =>{
                let left = stack.pop().expect("ERROR EVALUATING CONDITION");
                let right = stack.pop().expect("ERROR EVALUATING CONDITION");
                stack.push(Token::Or(Box::new(left), Box::new(right)))
            }
            "not" =>{
                let operand = stack.pop().expect("ERROR EVALUATING CONDITION");
                stack.push(Token::Not(Box::new(operand)))
            }
            num if is_number(num) => stack.push(Token::Number(num.to_string())),
            id if is_identifier(id) => stack.push(Token::Identifier(id.to_string())),
            _ => panic!("ERROR EVALUATING CONDITION"),
        }
    }
    if stack.len() != 1 {
        panic!("ERROR EVALUATING CONDITION");
    }
    stack.pop().expect("ERROR EVALUATING CONDITION")
}

fn parse_set_expression(input: Vec<&str>) -> Token {
    let mut stack: Vec<Token> = vec![];
    let mut comprehension:bool = false;
    if input[1] == ":"{
        comprehension = true;
    }
    if comprehension == false{
        for x in input{
            match x{
                "," =>continue,
                x if is_number(x)  => stack.push(Token::Number(x.to_string())),
                _=> stack.push(Token::Identifier(x.to_string())),

            }
        }
        Token::Set(stack)

    } else{

        let x = input[0];
        let input = shunting_yard(input[2..].to_vec());
        Token::Comprehension(Box::new(Token::Identifier(x.to_string())), Box::new(parse_condition(input)))

    }




}

fn parse_arithmetic_expression(input: Vec<&str>) -> Token {
    let input = shunting_yard(input);
    let mut stack: Vec<Token> = vec![];
    for token in input{
    match token {
        "+" => {
            let right = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            let left = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            stack.push(Token::Plus(Box::new(left), Box::new(right)))
        }
        "-" => {
            let right = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            let left = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            stack.push(Token::Minus(Box::new(left), Box::new(right)))
        }

        "*" => {
            let right = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            let left = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            stack.push(Token::Mult(Box::new(left), Box::new(right)))
        }

        "/" => {
            let right = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            let left = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            stack.push(Token::Div(Box::new(left), Box::new(right)))
        }

        "^" => {
            let right = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            let left = stack.pop().expect("ERROR EVALUATING MATH EXPRESSION");
            stack.push(Token::Pow(Box::new(left), Box::new(right)))
        }
        num if is_number(num)=> stack.push(Token::Number(num.to_string())),
        id if is_identifier(id) => stack.push(Token::Identifier(id.to_string())),
        _ => panic!("ERROR EVALUATING MATH EXPRESSION"),
    }
    }
    if stack.len() != 1 {
        panic!("ERROR EVALUATING MATH EXPRESSION");
    }
    Token::Expression(Box::new(stack.pop().expect("ERROR EVALUATING MATH EXPRESSION")))
}



fn parse_var_assignment(exp: Vec<&str>, line:u16 )-> Token{
if exp.len() == 0 {panic!("ERROR: UNREACHABLE {}", line) }
let variable_name = exp[1];
    let mut expression: Vec<&str> = vec![];


    for i in 3..exp.len() {
        expression.push(exp[i]);
    }
    Token::Assign(variable_name.to_string(),Box::new(parse_arithmetic_expression(expression)))


}
fn parse_set_assignment(exp: Vec<&str>, line:u16 )-> Token{
    if exp.len() == 0 {panic!("ERROR: UNREACHABLE {}", line) }
    let set_name = exp[1];
    let mut expression: Vec<&str> = vec![];
    
    for i in 4..exp.len() - 1{
        expression.push(exp[i]);
    }

    Token::Assign(set_name.to_string(),Box::new(parse_set_expression(expression)))
}
fn is_set(input: &Vec<&str>) -> bool {
    let mut input = input.clone();
    if let Some(x) = input.pop() {
        match x{
            "}" => true,
            _ => false,
        }
    } else {
        false
    }
}
fn parse_if(exp: Vec<&str>) -> Token{

    let condition:Token = parse_condition(shunting_yard(exp[1..].to_vec()));

    Token::If(Box::new(condition))


}
fn parse_assignment(exp: Vec<&str>,line: u16)-> Token {
    if  !is_set(&exp){
        parse_var_assignment(exp, line)
    } else {
        parse_set_assignment(exp, line)
    }
}
fn is_math_ex(input: &Vec<&str>) -> bool {
    for x in input {
        match x {
            &"+" | &"-" | &"*" | &"/" | &"^" => return true,
            _ => continue,
        }
    }
    false
}
fn scan(input:&str, line: u16) -> Token {
    if input.is_empty() {
        return Token::NewLine;
    }
    let mut expression: Vec<_> = input.split(" ").collect();
    expression.retain(|&x| x != "");
    println!("{:?}", expression);

    if is_math_ex(&expression) {
       parse_arithmetic_expression(expression)
    } else if expression.contains(&DEFINER[0]) {
        parse_map(expression, line)
    } else if expression.contains(&DEFINER[1]) {
        parse_assignment(expression, line)
    } else if expression.contains(&DEFINER[2]) {
        parse_if(expression)
    } else if expression.contains(&DEFINER[3]) {
        Token::End
    } else {
        panic!("ERROR: INVALID EXPRESSION ON LINE {}",line)
    }
}



pub fn tokenizer(input: String) -> Vec<Token> {
    let mut output: Vec<Token> = vec![];
    for (line, l) in input.lines().into_iter().enumerate() {
        let inp: Token = scan(l, line as u16);
        output.push(inp);
    }
    output
}
