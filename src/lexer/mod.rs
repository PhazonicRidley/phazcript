// lex a line of code

use std::fmt::Debug;

// TODO: add keywords, variables, etc
use crate::reconized_symbols;
pub mod datatype;
use datatype::Type;
#[derive(Debug, Clone)]
/**
 * A thing in the script. Right now can only be an operator or some data
 */
pub enum Token {
    Data(Type),
    Operator(String),
}

/**  TODO:
 * seperators include "" and '' (interchangable)
 * () is a AST that evaluates first then recursively is evaluated in the larger tree
 */
fn parse_seperators() {
    todo!("Complete")
}

pub fn determine_tokens(line: &str) -> Vec<Token> {
    // TODO: ignore white space, split tokens
    let line_list = line.split(" ");
    let mut determined_tokens: Vec<Token> = vec![];

    for unparsed_token in line_list {
        if unparsed_token.len() == 1
            && reconized_symbols::OPERATORS.contains(&unparsed_token.chars().nth(0).unwrap())
        {
            determined_tokens.push(Token::Operator(String::from(unparsed_token)));
        } else {
            match unparsed_token.parse::<i32>() // temp thing to only save numbers 
            {
                Ok(_ok) => determined_tokens.push(Token::Data(Type::new(unparsed_token))),
                Err(_e) => panic!("Not number")
            }
        }
    }
    return determined_tokens;
}
