use std::io::Result;
use std::fs;
use std::env;
mod lexer;
mod parser;
mod interpreter;

fn main() {
    // get contents -- get_file_from_args().unwrap();
    let contents = fs::read_to_string("src/calculation.txt").unwrap();

    // lexing
    let lexer_lines = lexer::get_lexer_lines(&contents);

    // parsing
    let parser_lines = parser::parse(lexer_lines);
    
    // interpreter
    interpreter::interpret(parser_lines);
}

#[allow(dead_code)]
fn debug_lexer(lexer_lines: Vec<lexer::Line>) {
    println!("LEXING:");
    for l in &lexer_lines {
        println!("  {}:", &l.number);
        for t in &l.tokens {
            println!("    {}: {}", t.token_type.to_string(), &t.value);
        }
    }
}

#[allow(dead_code)]
fn debug_parser(parser_lines: Vec<Option<parser::ExprNode>>) {
    println!("PARSING:");
    for l in &parser_lines {
        if l.is_some() {
            println!("{}", parser::print_expr(l.as_ref().unwrap()));
        }
    }
}

fn get_file_from_args() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Expects 1 argument being the file path");
        std::process::exit(1);
    } else {
        let file_path = &args[1];
        let content = fs::read_to_string(file_path)?;
        Ok(content)
    }
}