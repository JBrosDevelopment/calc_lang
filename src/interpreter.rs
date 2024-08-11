use crate::lexer::TokenType;
use crate::parser::ExprNode;
use std::io::stdin;
use rand::Rng;

pub struct PreDefinedFunction {
    pub name: String,
    pub parameters: i32,
    pub execute: fn(Vec<(f64, Option<Vec<f64>>)>) -> f64
}
#[derive(Debug, Clone)]
pub struct DefinedFunction {
    pub name: String,
    pub parameters: Option<Vec<String>>,
    pub function: Option<Box<ExprNode>>
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: (f64, Option<Vec<f64>>),
    pub is_constant: bool
}
#[derive(Debug, Clone)]
pub struct Marker {
    pub line_num: i32,
    pub mark: i32
}

impl DefinedFunction {
    pub fn new(name: String, parameters: Option<Vec<String>>, function: Option<Box<ExprNode>>) -> Self {
        DefinedFunction {
            name,
            parameters,
            function
        }
    }
}

impl Variable {
    pub fn new(name: String, value: (f64, Option<Vec<f64>>), is_constant: bool) -> Self {
        Variable {
            name,
            value,
            is_constant
        }
    }
}

/// Returns a vector of predefined functions
pub fn get_pre_defined_functions() -> Vec<PreDefinedFunction> {
    vec![
        PreDefinedFunction {
            name: "displayln".to_string(),
            parameters: 1,
            execute: |v| {
                println!("{}", v[0].0);
                0.0
            }
        },
        PreDefinedFunction {
            name: "display".to_string(),
            parameters: 1,
            execute: |v| {
                print!("{}", v[0].0);
                0.0
            }
        },
        PreDefinedFunction {
            name: "dacln".to_string(),
            parameters: 1,
            execute: |v| {
                unsafe {
                    println!("{}", char::from_u32_unchecked(v[0].0 as u32));
                    0.0
                }
            }
        },
        PreDefinedFunction {
            name: "dac".to_string(),
            parameters: 1,
            execute: |v| {
                unsafe {
                    print!("{}", char::from_u32_unchecked(v[0].0 as u32));
                    0.0
                }
            }
        },
        PreDefinedFunction {
            name: "read".to_string(),
            parameters: 0,
            execute: |_| {
                let mut buffer = String::new();
            
                stdin().read_line(&mut buffer).unwrap_or(0);
                match buffer.trim_end() {
                    "" => return FALSE,
                    out => return out.parse::<f64>().unwrap_or(0.0),
                };
            }
        },
        PreDefinedFunction {
            name: "clear".to_string(),
            parameters: 0,
            execute: |_| {
                println!("\x1B[2J\x1B[1;1H");
                0.0
            }
        },
        PreDefinedFunction {
            name: "len".to_string(),
            parameters: 1,
            execute: |x| {
                x[0].1.as_ref().unwrap().len() as f64
            }
        },
        PreDefinedFunction {
            name: "rand".to_string(),
            parameters: 0,
            execute: |x| {
                rand::random::<f64>() 
            }
        },
        PreDefinedFunction {
            name: "goto".to_string(),
            parameters: 1,
            execute: |v| {
                unsafe {
                    let mark = v[0].0 as i32;
                    let line = MARKERS.iter().find(|x| x.mark == mark).unwrap();
                    
                    NEXT_MARKER = line.clone();
                    SET_LINE = true;

                    line.line_num as f64
                }
            }
        }
    ]
}
/// Returns a vector of Variable structs initialized with default values from 'A' to 'Z'.
pub fn get_variables() -> Vec<Variable> {
    [
        Variable::new("A".to_string(), (FALSE, None), false),
        Variable::new("B".to_string(), (FALSE, None), false),
        Variable::new("C".to_string(), (FALSE, None), false),
        Variable::new("D".to_string(), (FALSE, None), false),
        Variable::new("E".to_string(), (FALSE, None), false),
        Variable::new("F".to_string(), (FALSE, None), false),
        Variable::new("G".to_string(), (FALSE, None), false),
        Variable::new("H".to_string(), (FALSE, None), false),
        Variable::new("I".to_string(), (FALSE, None), false),
        Variable::new("J".to_string(), (FALSE, None), false),
        Variable::new("K".to_string(), (FALSE, None), false),
        Variable::new("L".to_string(), (FALSE, None), false),
        Variable::new("M".to_string(), (FALSE, None), false),
        Variable::new("N".to_string(), (FALSE, None), false),
        Variable::new("O".to_string(), (FALSE, None), false),
        Variable::new("P".to_string(), (FALSE, None), false),
        Variable::new("Q".to_string(), (FALSE, None), false),
        Variable::new("R".to_string(), (FALSE, None), false),
        Variable::new("S".to_string(), (FALSE, None), false),
        Variable::new("T".to_string(), (FALSE, None), false),
        Variable::new("U".to_string(), (FALSE, None), false),
        Variable::new("V".to_string(), (FALSE, None), false),
        Variable::new("W".to_string(), (FALSE, None), false),
        Variable::new("X".to_string(), (FALSE, None), false),
        Variable::new("Y".to_string(), (FALSE, None), false),
        Variable::new("Z".to_string(), (FALSE, None), false),
    ].to_vec()
}
pub(crate) static mut MARKERS: Vec<Marker> = Vec::new();
static mut NEXT_MARKER: Marker = Marker { mark: 0, line_num: 0 };
static mut SET_LINE: bool = false;
const COLON_DEFAULT_FALSE_VALUE: f64 = -995413546.537209999;
const TRUE: f64 = 1.0;
const FALSE: f64 = 0.0;

/// # Interpret
/// Interprets the given lines of expressions using the default variables and an empty list of defined functions.
/// Calls the internal function 'interpret_with' with the provided lines, an empty list of defined functions, and default variables.
pub fn interpret(lines: Vec<Option<ExprNode>>) {
    interpret_with(lines, vec![], get_variables());
}

/// Interprets a series of lines of code, handling function definitions, variable assignments, and markers.
/// 
/// # Arguments
/// - `all_lines`: A vector of optional expression nodes representing the lines of code to interpret.
/// - `defined_functions`: A mutable vector of defined functions.
/// - `variables`: A mutable vector of variables.
/// 
/// # Safety
/// This function uses unsafe code to handle markers.
/// 
/// # Panics
/// - If an expected operand is missing or if a variable is not defined during assignment.
/// 
/// # Notes
/// The function recursively interprets lines and can skip lines based on markers.
pub fn interpret_with(all_lines: Vec<Option<ExprNode>>, mut defined_functions: Vec<DefinedFunction>, mut variables: Vec<Variable>) {
    let pre_defined_functions: Vec<PreDefinedFunction> = get_pre_defined_functions();
    let lines_binding = all_lines.clone();
    let lines= lines_binding.iter().cloned();
    
    for (_, line) in lines.enumerate() {
        if line.is_none() {
            continue;
        }
        if let Some(line) = line {
            let token = line.token.clone();
            match token.token_type {
                TokenType::DoubleArrow => {
                    if line.operand1.is_some() && line.operand2.is_some() {
                        // defining a function
                        let name = line.operand1.clone().unwrap().func_name.unwrap();
                        let parameters_full = line.operand1.clone().unwrap().func_parameters.unwrap();
                        let paremeters: Vec<String> = parameters_full.iter().map(|x| x[0].clone().unwrap().c).collect();
                        let function = line.operand2.clone().unwrap();

                        defined_functions.push(DefinedFunction::new(name, Some(paremeters), Some(function)));
                    }
                    else {
                        panic!("Expected operand1 and operand2 with operator {} in line {}", token.token_type.to_string(), line.line);
                    }
                }
                TokenType::Arrow => {
                    if line.operand1.is_some() && line.operand2.is_some() {
                        // assigning a variable
                        let value = solve_node(line.operand1.as_ref().unwrap(), &variables, &defined_functions, &pre_defined_functions);
                        let name = line.operand2.clone().unwrap().c;

                        if line.operand2.clone().unwrap().token.token_type == TokenType::Lowercase {
                            // defining constant
                            if variables.iter().any(|x| x.name == name) {
                                panic!("Constant {} can not be redefined in line {}", name, line.line);
                            }
                            variables.push(Variable::new(name, value, true));
                        }
                        else if let Some(index) = variables.iter().position(|x| x.name == name) {
                            // assigning variable
                            variables[index].value = value;
                        }
                        else {
                            panic!("Variable {} can not be defined in line {}", name, line.line);
                        }
                    }
                    else {
                        panic!("Expected operand1 and operand2 with operator {} in line {}", token.token_type.to_string(), line.line);
                    }
                }
                TokenType::DollarSign => {
                    /*let line_num = line.marker_line.unwrap();
                    let mark = line.marker_num.unwrap();
                    unsafe {
                        MARKERS.push(Marker {line_num, mark});
                    }*/
                    // assigned in parsing
                }
                _ => {
                    solve_node(&line, &variables, &defined_functions, &pre_defined_functions);
                }
            }
        }
        unsafe { 
            if SET_LINE {
                SET_LINE = false;
                let mut new_lines: Vec<Option<ExprNode>> = vec![];

                for (_, l) in all_lines.iter().enumerate() {
                    if let Some(l) = l {
                        if l.line >= NEXT_MARKER.line_num {
                            new_lines.push(Some(l.clone()));
                        }
                    }
                }
                
                interpret_with(new_lines, defined_functions, variables);
                break;
            }
        }
    }
}

/// Solves an expression node recursively based on its token type and operands.
/// 
/// # Arguments
/// - `node`: An expression node to be evaluated.
/// - `variables`: A vector of variables used in the expression.
/// - `defined_functions`: A vector of user-defined functions.
/// - `pre_defined_functions`: A vector of pre-defined functions.
/// 
/// # Returns
/// The result of the expression evaluation as a floating-point number.
pub fn solve_node(node: &ExprNode, variables: &Vec<Variable>, defined_functions: &Vec<DefinedFunction>, pre_defined_functions: &Vec<PreDefinedFunction>) -> (f64, Option<Vec<f64>>) {
    let mut left =  (FALSE, None);
    let mut right = (FALSE, None);
    if node.token.token_type.is_operator() {
        left = solve_node(node.operand1.as_ref().unwrap(), variables, defined_functions, pre_defined_functions);
        right = solve_node(node.operand2.as_ref().unwrap(), variables, defined_functions, pre_defined_functions);
    }
    match node.token.token_type {
        TokenType::Number => (node.token.value.parse().unwrap_or(0.0), None),
        TokenType::Plus => { 
            if left.1.is_some() || right.1.is_some() {
                panic!("Expected operand1 and operand2 to not be set with operator {} in line {}", node.token.token_type.to_string(), node.line);
            }
            (left.0 + right.0, None) 
        }
        TokenType::Dash => {
            if left.1.is_some() || right.1.is_some() {
                panic!("Expected operand1 and operand2 to not be set with operator {} in line {}", node.token.token_type.to_string(), node.line);
            } 
            (left.0 - right.0, None) 
        }
        TokenType::Star => {
            if left.1.is_some() || right.1.is_some() {
                panic!("Expected operand1 and operand2 to not be set with operator {} in line {}", node.token.token_type.to_string(), node.line);
            } 
            (left.0 * right.0, None) 
        }
        TokenType::Slash => {
            if left.1.is_some() || right.1.is_some() {
                panic!("Expected operand1 and operand2 to not be set with operator {} in line {}", node.token.token_type.to_string(), node.line);
            } 
            (left.0 / right.0, None) 
        }
        TokenType::Carrot => {
            if left.1.is_some() || right.1.is_some() {
                panic!("Expected operand1 and operand2 to not be set with operator {} in line {}", node.token.token_type.to_string(), node.line);
            } 
            (left.0.powf(right.0), None) 
        }
        TokenType::Percantage => {
            if left.1.is_some() || right.1.is_some() {
                panic!("Expected operand1 and operand2 to not be set with operator {} in line {}", node.token.token_type.to_string(), node.line);
            }
            (left.0 % right.0, None)
        }
        TokenType::Exclamation => {
            if left.1.is_some() || right.1.is_some() {
                panic!("Expected operand1 and operand2 to not be set with operator {} in line {}", node.token.token_type.to_string(), node.line);
            }
            (factorial(solve_node(node.operand1.as_ref().unwrap(), variables, defined_functions, pre_defined_functions).0), None)
        }
        TokenType::Equal => {
            if left.1.is_some() || right.1.is_some() {
                (bool_as_f64(left.1.unwrap().iter().zip(right.1.unwrap().iter()).all(|(x, y)| x == y)), None)
            }
            else {
                if left == right { (TRUE, None) }
                else { (FALSE, None) }
            }
        }
        TokenType::GreaterThan => {
            if left.1.is_some() || right.1.is_some() {
                (bool_as_f64(left.1.unwrap().iter().zip(right.1.unwrap().iter()).all(|(x, y)| x > y)), None)
            }
            else {
                if left > right { (TRUE, None) }
                else { (FALSE, None) }
            }
        }
        TokenType::GreaterThanOrEqualTo => {
            if left.1.is_some() || right.1.is_some() {
                (bool_as_f64(left.1.unwrap().iter().zip(right.1.unwrap().iter()).all(|(x, y)| x >= y)), None)
            }
            else {
                if left >= right { (TRUE, None) }
                else { (FALSE, None) }
            }
        }
        TokenType::LessThan => {
            if left.1.is_some() || right.1.is_some() {
                (bool_as_f64(left.1.unwrap().iter().zip(right.1.unwrap().iter()).all(|(x, y)| x < y)), None)
            }
            else {
                if left < right { (TRUE, None) }
                else { (FALSE, None) }
            }
        }
        TokenType::LessThanOrEqualTo => {
            if left.1.is_some() || right.1.is_some() {
                (bool_as_f64(left.1.unwrap().iter().zip(right.1.unwrap().iter()).all(|(x, y)| x <= y)), None)
            }
            else {
                if left <= right { (TRUE, None) }
                else { (FALSE, None) }
            }
        }
        TokenType::Uppercase => {
            let var_name = &node.token.value;
            for var in variables {
                if &var.name == var_name {
                    return var.value.clone();
                }
            }
            panic!("Variable '{}' does not exist in line {}", var_name, node.line);
        }
        TokenType::Lowercase => {
            if !node.is_func() {
                for var in variables {
                    if &var.name == &node.token.value {
                        return var.value.clone();
                    }
                }
                panic!("Variable '{}' does not exist in line {}", &node.token.value, node.line);
            }
            let func_name = node.func_name.clone().unwrap();
            if defined_functions.iter().any(|x| x.name == func_name) {
                let func = defined_functions.iter().find(|x| x.name == func_name).unwrap();
                let parameters = node.func_parameters.clone().unwrap();
                execute_defined_function(func, parameters, &variables, &defined_functions, &pre_defined_functions)
            }
            else if pre_defined_functions.iter().any(|x| x.name == func_name) {
                let func = pre_defined_functions.iter().find(|x| x.name == func_name).unwrap();
                let parameters = node.func_parameters.clone().unwrap();
                (execute_pre_defined_function(func, parameters, &variables, &defined_functions, &pre_defined_functions), None)
            }
            else {
                panic!("Function '{}' not defined in line {}", func_name, node.line);
            }
        }
        TokenType::Semicolon => {
            left = solve_node(node.operand1.as_ref().unwrap(), variables, defined_functions, pre_defined_functions);
            right = solve_node(node.operand2.as_ref().unwrap(), variables, defined_functions, pre_defined_functions);
            if left.0 == COLON_DEFAULT_FALSE_VALUE {
                return (right.0, None);
            }
            else {
                (left.0, None)
            }
        }
        TokenType::Colon => {
            left = solve_node(node.operand1.as_ref().unwrap(), variables, defined_functions, pre_defined_functions);
            if left.0 == TRUE {
                right = solve_node(node.operand2.as_ref().unwrap(), variables, defined_functions, pre_defined_functions);
                return (right.0, None);
            }
            else if left.0 == FALSE {
                return (COLON_DEFAULT_FALSE_VALUE, None);
            }
            panic!("Expected operand1 to have value 0 or 1 in line {}", node.line);
        }
        TokenType::OpenCurley => {
            if node.set.is_some() {
                let set = node.set.clone().unwrap();
                let mut result: Vec<f64> = vec![];
                for i in 0..set.len() {
                    result.push(solve_node(&set[i][0].as_ref().unwrap(), variables, defined_functions, pre_defined_functions).0);
                }
                return (0.0, Some(result));
            }
            else {
                panic!("Expected '{{' to be a set in line {}", node.line);
            }
        }
        TokenType::Underscore => {
            let left = solve_node(node.operand1.as_ref().unwrap(), variables, defined_functions, pre_defined_functions).1;
            let right = solve_node(node.operand2.as_ref().unwrap(), variables, defined_functions, pre_defined_functions).0;
            if right < 0.0 || right > left.clone().unwrap().len() as f64 {
                panic!("Index out of range in line {}", node.line);
            }
            (left.unwrap()[(right as usize) - 1], None)
        }
        _ => {
            panic!("Unsupported operation: {:?} in line {}", node.token.token_type, node.line);
        }
    }
}

/// Converts a boolean value to a floating-point number.
///
/// # Arguments
///
/// * `b` - A boolean value to be converted.
///
/// # Returns
///
/// A floating-point number representing the boolean value: 1.0 if `b` is true, 0.0 if `b` is false.
pub fn bool_as_f64(b: bool) -> f64 {
    if b { 1.0 } else { 0.0 }
}

/// Executes a defined function with the provided parameters, variables, and functions.
/// 
/// # Arguments
/// * `func` - A reference to the defined function to execute.
/// * `parameters` - A vector of vectors containing optional expression nodes as parameters.
/// * `variables` - A reference to a vector of variables.
/// * `defined_functions` - A reference to a vector of defined functions.
/// * `pre_defined_functions` - A reference to a vector of pre-defined functions.
/// 
/// # Returns
/// 
/// Returns the result of executing the defined function as a floating-point number (f64).
pub fn execute_defined_function(func: &DefinedFunction, parameters: Vec<Vec<Option<ExprNode>>>, variables: &Vec<Variable>, defined_functions: &Vec<DefinedFunction>, pre_defined_functions: &Vec<PreDefinedFunction>) -> (f64, Option<Vec<f64>>) {
    if parameters.len() != func.parameters.as_ref().unwrap().len() {
        panic!("Expected {} parameters, got {} in function: {}", func.parameters.as_ref().unwrap().len(), parameters.len(), func.name);
    }
    let mut params: Vec<(f64, Option<Vec<f64>>)> = Vec::new();
    for p in parameters {
        params.push(solve_node(&p[0].as_ref().unwrap(), variables.as_ref(), defined_functions, pre_defined_functions));
    }
    let mut param_vars: Vec<Variable> = Vec::new();
    for (i, p) in params.iter().enumerate() {
        param_vars.push(Variable::new(func.parameters.as_ref().unwrap()[i].clone(), p.clone(), false));
    }
    for constant in variables.iter().filter(|x| x.is_constant) {
        param_vars.push(constant.clone());
    }
    let node_box = func.function.clone();
    let top_node = *node_box.unwrap();
    solve_node(&top_node, &param_vars, defined_functions, pre_defined_functions)
}

/// Executes a pre-defined function with the provided parameters.
/// 
/// # Arguments
/// - `func`: A reference to the pre-defined function to be executed.
/// - `parameters`: A vector of vectors containing optional expression nodes as parameters.
/// - `variables`: A vector of variables used in the expression.
/// - `defined_functions`: A vector of defined functions.
/// - `pre_defined_functions`: A vector of pre-defined functions.
/// 
/// # Returns
/// The result of the function execution as a floating-point number.
pub fn execute_pre_defined_function(func: &PreDefinedFunction, parameters: Vec<Vec<Option<ExprNode>>>, variables: &Vec<Variable>, defined_functions: &Vec<DefinedFunction>, pre_defined_functions: &Vec<PreDefinedFunction>) -> f64 {
    if parameters.len() != func.parameters as usize {
        panic!("Expected {} parameters, got {} in function: {}", func.parameters, parameters.len(), func.name);
    }
    let mut params: Vec<(f64, Option<Vec<f64>>)> = Vec::new();
    for (i, param) in parameters.iter().enumerate() {
        params.push(solve_node(param[i].as_ref().unwrap(), variables, defined_functions, pre_defined_functions));
    }
    (func.execute)(params)
}

/// Calculates the factorial of a given number.
/// 
/// # Arguments
/// * `n` - The number to calculate the factorial of.
/// 
/// # Returns
/// The factorial of the given number.
pub fn factorial(n: f64) -> f64 {
    if n == FALSE {
        TRUE
    } else {
        n * factorial(n - TRUE)
    }
}