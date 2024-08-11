use core::panic;
use std::io::{self, Result};

#[derive(Clone, Debug, PartialEq)] 
pub(crate) enum TokenType {
    Arrow, Plus, Dash, Star, Slash, Equal, Percantage, Carrot,
    OpenParen, CloseParen, OpenCurley, CloseCurley, DoubleArrow,
    Colon, Semicolon, GreaterThan, LessThan, Comma, Dot, Exclamation,
    GreaterThanOrEqualTo, LessThanOrEqualTo, DollarSign, Underscore,
    Number, Uppercase, Lowercase, None
}
impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            TokenType::Arrow => "Arrow".to_string(),
            TokenType::DoubleArrow => "DoubleArrow".to_string(),
            TokenType::Plus => "Plus".to_string(),
            TokenType::Dash => "Dash".to_string(),
            TokenType::Star => "Star".to_string(),
            TokenType::Slash => "Slash".to_string(),
            TokenType::Equal => "Equal".to_string(),
            TokenType::Percantage => "Percantage".to_string(),
            TokenType::Carrot => "Carrot".to_string(),
            TokenType::OpenParen => "OpenParen".to_string(),
            TokenType::CloseParen => "CloseParen".to_string(),
            TokenType::OpenCurley => "OpenCurley".to_string(),
            TokenType::CloseCurley => "CloseCurley".to_string(),
            TokenType::Colon => "Colon".to_string(),
            TokenType::Semicolon => "Semicolon".to_string(),
            TokenType::GreaterThan => "GreaterThan".to_string(),
            TokenType::GreaterThanOrEqualTo => "GreaterThanOrEqualTo".to_string(),
            TokenType::LessThan => "LessThan".to_string(),
            TokenType::LessThanOrEqualTo => "LessThanOrEqualTo".to_string(),
            TokenType::Comma => "Comma".to_string(),
            TokenType::Dot => "Dot".to_string(),
            TokenType::Exclamation => "Exclamation".to_string(),
            TokenType::Number => "Number".to_string(),
            TokenType::Uppercase => "Uppercase".to_string(),
            TokenType::Lowercase => "Lowercase".to_string(),
            TokenType::None => "None".to_string(),
            TokenType::DollarSign => "DollarSign".to_string(),
            TokenType::Underscore => "Underscore".to_string(),
        }
    }
    pub fn is_operator(&self) -> bool {
        match self {
            TokenType::Arrow => false,
            TokenType::DoubleArrow => false,
            TokenType::Plus => true,
            TokenType::Dash => true,
            TokenType::Star => true,
            TokenType::Slash => true,
            TokenType::Equal => true,
            TokenType::Percantage => true,
            TokenType::Carrot => true,
            TokenType::OpenParen => false,
            TokenType::CloseParen => false,
            TokenType::OpenCurley => false,
            TokenType::CloseCurley => false,
            TokenType::Colon => false,
            TokenType::Semicolon => false,
            TokenType::GreaterThan => true,
            TokenType::GreaterThanOrEqualTo => true,
            TokenType::LessThan => true,
            TokenType::LessThanOrEqualTo => true,
            TokenType::Comma => false,
            TokenType::Dot => false,
            TokenType::Exclamation => false,
            TokenType::Number => false,
            TokenType::Uppercase => false,
            TokenType::Lowercase => false,
            TokenType::None => false,
            TokenType::DollarSign => false,
            TokenType::Underscore => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub tokens: Vec<Token>, pub number: i32
}
#[derive(Clone, Debug, PartialEq)] 
pub struct Token {
    pub token_type: TokenType, pub value: String
}
/// Processes the contents to extract lexer lines.
/// 
/// # Arguments
/// * `contents` - A string containing the contents to be processed.
/// 
/// # Returns
/// A vector of `Line` structs representing the lexer lines extracted from the contents.
pub fn get_lexer_lines(contents: &str) -> Vec<Line> {
    let lines: Vec<&str> = contents.split("\n").filter(|&x| x.trim() != "").collect();
    let mut lexer_lines: Vec<Line> = Vec::new();
    
    // return lexer lines
    let mut line_number = 0;
    for line in lines {
        line_number += 1;
        let l = get_lexer_line(line, line_number);
        lexer_lines.push(l);
    }
    lexer_lines
}

/// Parses a line of text into a collection of tokens representing different types of symbols, numbers, and characters.
/// 
/// # Arguments
/// * `line` - The input line of text to be parsed.
/// * `line_number` - The line number of the input line.
/// 
/// # Returns
/// A `Line` struct containing the parsed tokens and the line number.
pub fn get_lexer_line(line: &str, line_number: i32, ) -> Line {
    let chars: Vec<&str> = line.split("").collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut single: bool = false;
    let mut skip: usize = 0;
    let mut number: String = String::new();
    let mut alphabetical: String = String::new();

    // loop through chars
    for (j, &c) in chars.iter().enumerate() {
        if skip > 0 { 
            skip -= 1;
            continue;
        }

        // is number
        if let Ok(_) = parse_str_to_i32(c) {
            number += c;
        }
        // is alphabetical
        else if is_alphabetical(c) {
            alphabetical += c;
        }
        // is dot
        else if c == "." {
            // and is number
            if number != "".to_string() {
                number += c;
            }
            // and next is number
            else if chars.len() > j + 1 && parse_str_to_i32(&chars[j + 1].to_string()).is_ok() {
                number += c;
            }
            // and is apart of comment
            else if chars.len() > j + 2 && chars[j + 1] == "." && chars[j + 2] == "." {
                break;
            }
            // and by itself 
            else {
                tokens.push(Token { token_type: TokenType::Dot, value: c.to_string() });
                single = true;
            }
        }
        // is dash
        else if c == "-" {
            // and apart of arrow
            if chars.len() > j + 1 && chars[j + 1] == ">" {
                tokens.push(Token { token_type: TokenType::Arrow, value: "->".to_string() });
                skip = 2;
            }
            // and by itself 
            else {
                tokens.push(Token { token_type: TokenType::Dash, value: c.to_string() });
                single = true;
            }
        }
        // is dash
        else if c == "-" {
            // and apart of arrow
            if chars.len() > j + 1 && chars[j + 1] == ">" {
                tokens.push(Token { token_type: TokenType::Arrow, value: "->".to_string() });
                skip = 2;
            }
            // and by itself 
            else {
                tokens.push(Token { token_type: TokenType::Dash, value: c.to_string() });
                single = true;
            }
        }
        // is equal
        else if c == "=" {
            // and apart of double arrow
            if chars.len() > j + 1 && chars[j + 1] == ">" {
                tokens.push(Token { token_type: TokenType::DoubleArrow, value: "=>".to_string() });
                skip = 2;
            }
            // and by itself 
            else {
                tokens.push(Token { token_type: TokenType::Equal, value: c.to_string() });
                single = true;
            }
        }
        // is less than
        else if c == "<" {
            // and apart of less than or equal to
            if chars.len() > j + 1 && chars[j + 1] == "=" {
                tokens.push(Token { token_type: TokenType::LessThanOrEqualTo, value: "<=".to_string() });
                skip = 2;
            }
            // and by itself 
            else {
                tokens.push(Token { token_type: TokenType::LessThan, value: c.to_string() });
                single = true;
            }
        }
        // is less than
        else if c == ">" {
            // and apart of greater than or equal to
            if chars.len() > j + 1 && chars[j + 1] == "=" {
                tokens.push(Token { token_type: TokenType::GreaterThanOrEqualTo, value: ">=".to_string() });
                skip = 2;
            }
            // and by itself 
            else {
                tokens.push(Token { token_type: TokenType::GreaterThan, value: c.to_string() });
                single = true;
            }
        }
        // symbols:
        else if c == "+" { tokens.push(Token { token_type: TokenType::Plus, value: c.to_string() }); single = true; }
        else if c == "*" { tokens.push(Token { token_type: TokenType::Star, value: c.to_string() }); single = true; }
        else if c == "/" { tokens.push(Token { token_type: TokenType::Slash, value: c.to_string() }); single = true; }
        else if c == "^" { tokens.push(Token { token_type: TokenType::Carrot, value: c.to_string() }); single = true; }
        else if c == "%" { tokens.push(Token { token_type: TokenType::Percantage, value: c.to_string() }); single = true; }
        else if c == "!" { tokens.push(Token { token_type: TokenType::Exclamation, value: c.to_string() }); single = true; }
        else if c == "(" { tokens.push(Token { token_type: TokenType::OpenParen, value: c.to_string() }); single = true; }
        else if c == ")" { tokens.push(Token { token_type: TokenType::CloseParen, value: c.to_string() }); single = true; }
        else if c == "{" { tokens.push(Token { token_type: TokenType::OpenCurley, value: c.to_string() }); single = true; }
        else if c == "}" { tokens.push(Token { token_type: TokenType::CloseCurley, value: c.to_string() }); single = true; }
        else if c == ";" { tokens.push(Token { token_type: TokenType::Semicolon, value: c.to_string() }); single = true; }
        else if c == ":" { tokens.push(Token { token_type: TokenType::Colon, value: c.to_string() }); single = true; }
        else if c == "," { tokens.push(Token { token_type: TokenType::Comma, value: c.to_string() }); single = true; }
        else if c == "$" { tokens.push(Token { token_type: TokenType::DollarSign, value: c.to_string() }); single = true; }
        else if c == "_" { tokens.push(Token { token_type: TokenType::Underscore, value: c.to_string() }); single = true; }
        // spaces:
        if c == " " || chars.len() == j + 1 || single {
            let mut tok: Token = Token { token_type:TokenType::None, value:String::new() };
            if single {
                tok = tokens.last().unwrap().clone();
                tokens.remove(tokens.len() - 1);
            }
            if !number.is_empty() {
                tokens.push(Token { token_type: TokenType::Number, value: number.to_string() });
                number = String::new();
            }
            if !alphabetical.is_empty() {
                if alphabetical.chars().all(|c| c.is_uppercase()) {
                    for alph in alphabetical.chars() {
                        tokens.push(Token { token_type: TokenType::Uppercase, value: alph.to_string() });
                    }
                }
                else if alphabetical.chars().all(|c| c.is_lowercase()) {
                    tokens.push(Token { token_type: TokenType::Lowercase, value: alphabetical.to_string() });
                }
                else {
                    panic!("Unkown token_type '{}', expects either function (all lowercase) or variable (all uppercase)", alphabetical);
                }
                alphabetical = String::new();
            }
            if single {
                single = false;
                tokens.push(tok);
            }
        }
    }

    Line { tokens:tokens, number:line_number }
}

/// Parses a string into an i32 value.
/// 
/// # Arguments
/// 
/// * `s` - A reference to the input string to be parsed.
/// 
/// # Returns
/// 
/// * `Result<i32>` - An `Ok` variant containing the parsed i32 value if successful, or an `Err` variant with an `io::Error` if parsing fails.
pub fn parse_str_to_i32(s: &str) -> Result<i32> {
    match s.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

/// Checks if the input string contains a single alphabetical character.
/// 
/// # Arguments
/// 
/// * `s` - A string slice to be checked.
/// 
/// # Returns
/// 
/// * `true` if the string contains a single alphabetical character, `false` otherwise.
/// 
/// # Panics
/// 
/// Panics if the input string is not a single character, otherwise returns `false`.
pub fn is_alphabetical(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    if s.len() == 1 {
        match s.parse::<char>() {
            Ok(c) => return c.is_alphabetic(),
            Err(_) => return false
        };
    }
    else {
        return false;
    }
}