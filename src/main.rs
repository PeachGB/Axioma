use std::io;
mod lexer;
fn main() -> io::Result<()> {
    let input: String = String::from("41 + 555;\n4554789 + 123");
    let lexer = lexer::scan_token(input);
    let lexer = lexer::evaluator(lexer);
    println!("{:?}",lexer);
    Ok(())

}
