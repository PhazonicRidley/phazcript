// lex a line of code

use std::{fmt::Debug, ops};

// TODO: add keywords, variables, etc
use crate::reconized_symbols;

#[derive(Debug, Clone)]
/**
 * A thing in the script. Right now can only be an operator or some data
 */
pub enum Token {
    Data(DataValue),
    Operator(String)
}
#[derive(Debug, Clone)]
/**
 * A wrapper struct for data 
 * Probably going to wrap each type itself and turn this thing into a trait
 */
pub struct DataValue
{
    pub value: Type,
    pub d_type: String
}

impl DataValue {
    fn determine_type(value: &String) -> Option<(Type, String)>
    {
        let mut d_type: Option<(Type, String)> = None;
        if let Ok(num) = value.parse::<i32>()
        {
            d_type = Some((Type::Numeric(num), String::from("Numeric"))) // make less shit
        }
        else if let Ok(c) = value.parse::<char>()
        {
            d_type = Some((Type::Char(c), String::from("Char")))
        }
        else if let Ok(bool) = value.parse::<bool>()
        {
            d_type = Some((Type::Bool(bool), String::from("Bool")))
        }
        else 
        {
            d_type = Some((Type::String(value.clone()), String::from("String")))
        }
        return d_type
    }

    fn new(val: &str) -> DataValue {
        let data_wrapper = DataValue::determine_type(&val.to_owned()).expect("Unable to determine datatype");

        return DataValue { value: data_wrapper.0, d_type: data_wrapper.1 }
        
    }
}

#[derive(Debug, Clone)]
/**
 * Enum for different data types
 */
pub enum Type
{
    String(String),
    Char(char),
    Numeric(i32),
    Bool(bool)
}

impl ops::Add for Type
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        match (self, other)
        {
            (Type::Numeric(first), Type::Numeric(second)) => Type::Numeric(first + second),
            (Type::Char(first), Type::Char(second)) => Type::String(format!("{}{}", first, second)),
            (Type::String(first), Type::String(second)) => Type::String(format!("{}{}", first, second)),
            _ => panic!("Unable to add")
        }
    }
}

impl ops::Sub for Type
{
    type Output = Self;

    fn sub(self, other: Self) -> Self
    {
        if let (Type::Numeric(first), Type::Numeric(second)) = (self, other)
        {
            return Type::Numeric(first - second)
        }
        else {
            panic!("Cannot subtract types")
        }
    }
}

pub fn determine_tokens(line: &str) -> Vec<Token>
{
    let line_list = line.split(" ");
    let mut determined_tokens: Vec<Token> = vec![];

    for unparsed_token in line_list
    {
        if unparsed_token.len() == 1 && reconized_symbols::OPERATORS.contains(&unparsed_token.chars().nth(0).unwrap())
        {
            determined_tokens.push(Token::Operator(String::from(unparsed_token)));
        }
        else
        {
            match unparsed_token.parse::<i32>() // temp thing to only save numbers 
            {
                Ok(_ok) => determined_tokens.push(Token::Data(DataValue::new(unparsed_token))),
                Err(_e) => panic!("Not number")
            }
        }
    }
    return determined_tokens
}