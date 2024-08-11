use crate::lexer::{Line, Token, TokenType};
use crate::interpreter::{Marker, MARKERS};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct ExprNode {
    pub c: String,
    pub token: Token,
    pub line: i32,
    pub operand1: Option<Box<ExprNode>>,
    pub operand2: Option<Box<ExprNode>>,
    pub func_parameters: Option<Vec<Vec<Option<ExprNode>>>>,
    pub func_name: Option<String>,
    pub marker_line: Option<i32>,
    pub marker_num: Option<i32>,
    pub set: Option<Vec<Vec<Option<ExprNode>>>>
}

impl ExprNode {
    pub fn is_func(&self) -> bool {
        self.func_name.is_some() && self.func_parameters.is_some() 
    }
    fn new_num(num: Token, line: i32) -> Self {
        ExprNode {
            c: num.value.clone(),
            token: num,
            line: line,
            operand1: None,
            operand2: None,
            func_parameters: None,
            func_name: None,
            marker_line: None,
            marker_num: None,
            set: None
        }
    }
    fn new_op(op: Token, e1: ExprNode, e2: ExprNode, line: i32) -> Self {
        ExprNode {
            c: op.value.clone(),
            token: op,
            line: line,
            operand1: Some(Box::new(e1)),
            operand2: Some(Box::new(e2)),
            func_parameters: None,
            func_name: None,
            marker_line: None,
            marker_num: None,
            set: None
        }
    }
    fn new_func(func: Token, parameters: Option<Vec<Vec<Option<ExprNode>>>>, line: i32) -> Self {
        ExprNode {
            c: String::new(),
            token: func.clone(),
            line: line,
            operand1: None,
            operand2: None,
            func_parameters: parameters,
            func_name: Some(func.value.clone()),
            marker_line: None,
            marker_num: None,
            set: None
        }
    }
    fn new_marker(token: Token, line: i32, number: i32) -> Self {
        ExprNode {
            c: String::new(),
            token: token.clone(),
            line: line,
            operand1: None,
            operand2: None,
            func_parameters: None,
            func_name: None,
            marker_line: Some(line.clone()),
            marker_num: Some(number.clone()),
            set: None
        }
    }
    fn new_set(token: Token, indexes: Option<Vec<Vec<Option<ExprNode>>>>, line_num: i32) -> Self {
        ExprNode {
            c: String::new(),
            token: token.clone(),
            line: line_num,
            operand1: None,
            operand2: None,
            func_parameters: None,
            func_name: None,
            marker_line: None,
            marker_num: None,
            set: indexes.clone()
        }
    }
}

pub fn precedence(op: String) -> i32 {
    match op.as_str() {
        "->"| "=>" => -1,
        ";" => 1,
        ":" => 3,
        "=" => 3,
        ">" | "<" | ">=" | "<=" => 4,
        "+" | "-" => 5,
        "*" | "/" => 6,
        "^" | "_" => 7,
        "%" => 8,
        _ => 0,
    }
}

/// Parses a list of lexer lines into an expression tree.
/// 
/// # Arguments
/// * `lexer_lines` - A vector of lexer lines to be parsed.
/// 
/// # Returns
/// A vector of optional expression nodes representing the parsed input.
pub fn parse(lexer_lines: Vec<Line>) -> Vec<Option<ExprNode>> {
    let mut lines = lexer_lines.iter().peekable();
    let mut returns: Vec<Option<ExprNode>> = Vec::new();

    while let Some(l) = lines.peek() {
        let mut chars = l.tokens.iter().peekable();
        let char_len = chars.clone().count();
        let mut last_was_digit_or_closing = false;
        let mut last_was_variable = false;
        let mut operator_stack: Vec<Token> = Vec::new();
        let mut expr_stack: Vec<ExprNode> = Vec::new();
        let number = l.number;

        while let Some(c) = chars.peek() {
            match c.token_type {
                TokenType::OpenParen => {
                    if last_was_digit_or_closing || last_was_variable {
                        // Implicit multiplication: e.g., "2(" or ")("
                        operator_stack.push(Token { value: "*".to_string(), token_type: TokenType::Star });
                    }
                    operator_stack.push((*c).clone());
                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::Number => {
                    expr_stack.push(ExprNode::new_num((*c).clone(), number));
                    chars.next();
                    last_was_digit_or_closing = true;
                    last_was_variable = false;
                }
                TokenType::Dash => {
                    if !last_was_digit_or_closing && !last_was_variable {
                        chars.next();
                        let operator = Token { value: "-".to_string(), token_type: TokenType::Dash };
                        let e1 = ExprNode::new_num(Token { token_type: TokenType::Number, value: "0".to_string() }, number);
                        let e2 = ExprNode::new_num(chars.peek().unwrap().clone().clone(), number);
                        expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
                    } else {
                        while operator_stack.last().map_or(false, |top| precedence(top.value.clone()) >= precedence(c.value.clone())) && expr_stack.len() > 1 {
                            let operator = operator_stack.pop().unwrap();
                            let e2 = expr_stack.pop().unwrap();
                            let e1 = expr_stack.pop().unwrap();
                            expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
                        }
                        operator_stack.push((*c).clone());
                    }
                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::Plus | TokenType::Star | TokenType::Slash | TokenType::Arrow | TokenType::Equal | TokenType::Percantage | 
                TokenType::Carrot | TokenType::Colon | TokenType::GreaterThan | TokenType::GreaterThanOrEqualTo | TokenType::LessThan | TokenType::LessThanOrEqualTo | 
                TokenType::Comma | TokenType::Semicolon | TokenType::Underscore | TokenType::DoubleArrow => {
                    while operator_stack.last().map_or(false, |top| precedence(top.value.clone()) >= precedence(c.value.clone())) && expr_stack.len() > 1 {
                        let operator = operator_stack.pop().unwrap();
                        let e2 = expr_stack.pop().unwrap();
                        let e1 = expr_stack.pop().unwrap();
                        expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
                    }
                    operator_stack.push((*c).clone());
                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::Exclamation => {
                    let operator = Token { value: "!".to_string(), token_type: TokenType::Exclamation };
                    let e1 = expr_stack.pop().unwrap();
                    let e2 = ExprNode::new_num(Token { token_type: TokenType::None, value: String::new() }, number);
                    expr_stack.push(ExprNode::new_op(operator, e1, e2, number));

                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::CloseParen => {
                    while operator_stack.last().map_or(false, |op| op.token_type != TokenType::OpenParen) && expr_stack.len() > 1 {
                        let operator = operator_stack.pop().unwrap();
                        let e2 = expr_stack.pop().unwrap();
                        let e1 = expr_stack.pop().unwrap();
                        expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
                    }
                    operator_stack.pop(); // Pop the '('
                    chars.next();
                    last_was_digit_or_closing = true;
                    last_was_variable = false;
                }
                TokenType::Uppercase => {
                    if last_was_digit_or_closing || last_was_variable {
                        operator_stack.push(Token { value: "*".to_string(), token_type: TokenType::Star });
                    }
                    expr_stack.push(ExprNode::new_num((*c).clone(), number));
                    chars.next();
                    last_was_digit_or_closing = true;
                    last_was_variable = true;
                }
                TokenType::Lowercase => {
                    // Get all tokens inside the parentheses
                    let mut params_tokens_inside_parens: Vec<Vec<Token>> = vec![vec![]];
                    let mut params_index = 0;
                    let mut parenthesis_count = 0;
                    let mut index = 0;
                    let name = (*c).clone();
                    chars.next();

                    while let Some(c) = chars.peek() {
                        match c.token_type {
                            TokenType::OpenParen => {
                                index += 1;
                                parenthesis_count += 1;
                                if parenthesis_count > 1 {
                                    params_tokens_inside_parens[params_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            TokenType::CloseParen => {
                                if index == 0 {
                                    break;
                                }
                                index += 1;
                                parenthesis_count -= 1;
                                let mut breaks = false;
                                if parenthesis_count == 0 {
                                    breaks = true;
                                }
                                else {
                                    params_tokens_inside_parens[params_index].push((*c).clone());
                                }
                                chars.next();
                                
                                if breaks{
                                    break;
                                }
                            }
                            TokenType::Comma => {
                                if index == 0 {
                                    break;
                                }
                                index += 1;
                                if parenthesis_count == 1 {
                                    params_tokens_inside_parens.push(vec![]);
                                    params_index += 1;
                                }
                                else {
                                    params_tokens_inside_parens[params_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            _ => {
                                if index == 0 {
                                    break;
                                }
                                index += 1;
                                params_tokens_inside_parens[params_index].push((*c).clone());
                                chars.next();
                            }
                        }
                    }

                    if index == 0 {
                        // is constant
                        if last_was_digit_or_closing || last_was_variable {
                            operator_stack.push(Token { value: "*".to_string(), token_type: TokenType::Star });
                        }

                        expr_stack.push(ExprNode::new_num(name.clone(), number));
                        last_was_digit_or_closing = true;
                        last_was_variable = true;
                    }
                    else {
                        // is function
                        let mut parsed_tokens: Vec<Vec<Option<ExprNode>>> = Vec::new();
                        if params_tokens_inside_parens[0].len() != 0 {   
                            for tokens_inside_parens in params_tokens_inside_parens {
                                parsed_tokens.push(parse(vec![Line { number: l.number, tokens: tokens_inside_parens }]));
                            }
                        }
                        
                        expr_stack.push(ExprNode::new_func(name, Some(parsed_tokens), number));
                        
                        last_was_digit_or_closing = true;
                        last_was_variable = false;
                    }
                }
                TokenType::DollarSign => {
                    if !operator_stack.is_empty() && !expr_stack.is_empty() {
                        panic!("Expected '$' to be at the start of line in line {}", number);
                    }
                    if char_len > 2 {
                        panic!("marker_line syntax is incorrect. Expected '$' to be at the start of line and number after it: '$0' in line {}", number);
                    }
                    let token = c.clone();
                    chars.next();
                    let mark = chars.next().unwrap().value.clone().parse::<i32>().unwrap();
                    unsafe { MARKERS.push(Marker {line_num: l.number, mark}) };
                    
                    expr_stack.push(ExprNode::new_marker(token.clone(), number, mark));
                    break;
                }
                TokenType::OpenCurley => {
                    // Get all tokens inside the curleys
                    let mut curley_tokens_inside_parens: Vec<Vec<Token>> = vec![vec![]];
                    let mut curley_index = 0;
                    let mut curley_count = 1;
                    let token = (*c).clone();
                    chars.next();

                    while let Some(c) = chars.peek() {
                        match c.token_type {
                            TokenType::OpenCurley => {
                                curley_count += 1;
                                if curley_count > 1 {
                                    curley_tokens_inside_parens[curley_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            TokenType::CloseCurley => {
                                curley_count -= 1;
                                let mut breaks = false;
                                if curley_count == 0 {
                                    breaks = true;
                                }
                                else {
                                    curley_tokens_inside_parens[curley_index].push((*c).clone());
                                }
                                chars.next();
                                
                                if breaks{
                                    break;
                                }
                            }
                            TokenType::Comma => {
                                if curley_count == 1 {
                                    curley_tokens_inside_parens.push(vec![]);
                                    curley_index += 1;
                                }
                                else {
                                    curley_tokens_inside_parens[curley_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            _ => {
                                curley_tokens_inside_parens[curley_index].push((*c).clone());
                                chars.next();
                            }
                        }
                    }

                    let mut parsed_tokens: Vec<Vec<Option<ExprNode>>> = Vec::new();
                    if curley_tokens_inside_parens[0].len() != 0 {   
                        for tokens_inside_parens in curley_tokens_inside_parens {
                            parsed_tokens.push(parse(vec![Line { number: l.number, tokens: tokens_inside_parens }]));
                        }
                    }
                    
                    expr_stack.push(ExprNode::new_set(token, Some(parsed_tokens), number));
                    
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::CloseCurley => {
                    panic!("Unexpected character '{}', has '}}' without opening '{{' in line {}", c.value, number); 
                }
                _ => {
                    panic!("Unexpected character '{}' in line {}", c.value, number); // Error handling
                }
            }
        }

        let mut i = 0;
        while let Some(operator) = operator_stack.pop() {
            i += 1;
            if i < 99 {
                let mut e1 = ExprNode::new_num(Token { value: String::new(), token_type: TokenType::None }, number);
                let mut e2 = ExprNode::new_num(Token { value: String::new(), token_type: TokenType::None }, number);
                if !expr_stack.is_empty() {
                    e2 = expr_stack.pop().unwrap();
                }
                if !expr_stack.is_empty() {
                    e1 = expr_stack.pop().unwrap();
                }
                expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
            }
        }

        returns.push(expr_stack.pop());
        lines.next();
    }
    returns
}

fn index_is_none(index: &Option<Token>) -> bool {
    index.is_none()
}

/// Prints the expression represented by the given ExprNode.
/// 
/// # Arguments
/// * `node` - An ExprNode reference containing the expression to be printed.
/// 
/// # Returns
/// A String representing the printed expression.
pub fn print_expr(node: &ExprNode) -> String {
    if node.func_name.is_some() && node.func_parameters.is_some() {
        let mut func_params: String = String::new();
        for n in node.func_parameters.as_ref().unwrap() {
            for o in n.iter() {
                if let Some(p) = o.as_ref() {
                    func_params = format!("{}, {}", func_params, print_expr(p));
                }
            }
        }

        return format!("{}({})", node.func_name.as_ref().unwrap(), func_params.trim_start_matches(", "));
    }

    if node.marker_line.is_some() {
        return format!("${}", node.marker_num.as_ref().unwrap());
    }

    if node.operand1.is_none() && node.operand2.is_none() {
        return node.c.to_string();
    }

    let left = node.operand1.as_ref().map_or("".to_string(), |n| print_expr(n));
    let right = node.operand2.as_ref().map_or("".to_string(), |n| print_expr(n));

    format!("({}{}{})", left, node.c, right)
}
