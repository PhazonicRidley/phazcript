// lex a line of code

use std::{fmt::Debug, ops};

// TODO: add keywords, variables, etc
use crate::reconized_symbols;

#[derive(Debug, Clone)]
/**
 * A thing in the script. Right now can only be an operator or some data
 */
pub enum Token {
    Data(Type),
    Operator(String),
}

#[derive(Debug, Clone)]
/**
 * Enum for different data types
 */
// TODO: Seperate out into its own file
pub enum Type {
    String(String),
    Numeric(i32),
    Bool(bool),
}

impl Type {
    fn determine(value: &str) -> Option<Self> {
        let d_type: Option<Self>;
        if let Ok(num) = value.parse::<i32>() {
            d_type = Some(Type::Numeric(num))
        } else if let Ok(b) = value.parse::<bool>() {
            d_type = Some(Type::Bool(b))
        } else {
            d_type = Some(Type::String(value.to_owned()))
        }
        if !d_type.is_some() {
            return None;
        }
        return d_type;
    }

    fn new(value: &str) -> Self {
        let d_type = Type::determine(value);
        return d_type.expect("Unable to determine datatype.");
    }
}

impl ops::Add for Type {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Type::Numeric(first), Type::Numeric(second)) => Type::Numeric(first + second),
            (Type::String(first), Type::String(second)) => {
                Type::String(format!("{}{}", first, second))
            }
            _ => panic!("Unable to add"),
        }
    }
}

impl ops::Sub for Type {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if let (Type::Numeric(first), Type::Numeric(second)) = (self, other) {
            return Type::Numeric(first - second);
        } else {
            panic!("Cannot subtract types")
        }
    }
}

impl ops::Mul for Type {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if let (Type::Numeric(first), Type::Numeric(second)) = (self, other) {
            return Type::Numeric(first * second);
        } else {
            panic!("Cannot multiply types")
        }
    }
}

impl ops::Div for Type {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if let (Type::Numeric(first), Type::Numeric(second)) = (self, other) {
            if second == 0 {
                panic!("Cannot divide by zero.")
            }
            return Type::Numeric(first / second);
        } else {
            panic!("Cannot divide types")
        }
    }
}

fn parse_seperators() {}

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
