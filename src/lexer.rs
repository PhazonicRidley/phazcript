// lex a line of code


use std::process::exit;

// TODO: add keywords, variables, etc
use crate::reconized_symbols;

#[derive(Debug)]
pub enum Token {
    Data(DataValue),
    Operator(String)
}
#[derive(Debug)]
struct DataValue
{
    value: Data,
    d_type: String
}

impl DataValue {
    fn determine_type(value: &String) -> Option<(Data, String)>
    {
        let mut d_type: Option<(Data, String)> = None;
        if let Ok(num) = value.parse::<i32>()
        {
            d_type = Some((Data::numeric(num), String::from("numerical"))) // make less shit
        }
        else if let Ok(c) = value.parse::<char>()
        {
            d_type = Some((Data::char(c), String::from("char")))
        }
        else if let Ok(bool) = value.parse::<bool>()
        {
            d_type = Some((Data::bool(bool), String::from("bool")))
        }
        else 
        {
            d_type = Some((Data::string(value.clone()), String::from("string")))
        }
        return d_type
    }

    fn new(val: &str) -> DataValue {
        let data_wrapper = DataValue::determine_type(&val.to_owned()).expect("Unable to determine datatype");

        return DataValue { value: data_wrapper.0, d_type: data_wrapper.1 }
        
    }
}

#[derive(Debug)]
enum Data
{
    string(String),
    char(char),
    numeric(i32),
    bool(bool)
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