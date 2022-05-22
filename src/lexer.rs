// lex a line of code

use std::fmt::{Display, Debug};

// TODO: add keywords, variables, etc
use crate::reconized_symbols;

#[derive(Debug)]
/**
 * A thing in the script. Right now can only be an operator or some data
 */
pub enum Token {
    Data(DataValue),
    Operator(String)
}
#[derive(Debug)]
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
            d_type = Some((Type::Numeric(num), String::from("numerical"))) // make less shit
        }
        else if let Ok(c) = value.parse::<char>()
        {
            d_type = Some((Type::Char(c), String::from("char")))
        }
        else if let Ok(bool) = value.parse::<bool>()
        {
            d_type = Some((Type::Bool(bool), String::from("bool")))
        }
        else 
        {
            d_type = Some((Type::String(value.clone()), String::from("string")))
        }
        return d_type
    }

    fn new(val: &str) -> DataValue {
        let data_wrapper = DataValue::determine_type(&val.to_owned()).expect("Unable to determine datatype");

        return DataValue { value: data_wrapper.0, d_type: data_wrapper.1 }
        
    }
}

#[derive(Debug)]
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

pub trait Test {
    fn make_string(&self) -> String where Self: Debug + Sized
    {
        format!("This is a {:?}", self)
    }
}

impl Test for i32 {}
impl Test for char {}
impl Test for String {}
impl Test for bool {}
impl Test for Type {}

pub fn trait_test(thing: Type)
{
    println!("{}", thing.make_string())
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
                Ok(ok) => determined_tokens.push(Token::Data(DataValue::new(unparsed_token))),
                Err(e) => panic!("Not number")
            }
        }
    }
    return determined_tokens
}