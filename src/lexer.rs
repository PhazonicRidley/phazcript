// lex a line of code


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
    value: String,
    d_type: String
}

impl DataValue {
    fn determine_type(value: &String) -> Option<String>
    {
        let mut d_type: Option<String> = None;
        if let Ok(num) = value.parse::<f64>()
        {
            d_type = Some(String::from("numerical")) // make less shit
        }
        else if let Ok(c) = value.parse::<char>()
        {
            d_type = Some(String::from("char"))
        }
        else if let Ok(bool) = value.parse::<bool>()
        {
            d_type = Some(String::from("bool"))
        }
        else 
        {
            d_type = Some(String::from("string"))
        }
        return d_type
    }

    fn new(val: &str) -> DataValue {
        let d_t = DataValue::determine_type(&val.to_owned()).expect("Unable to determine variable type!"); // should never activate
        let data_value = DataValue {
            value: val.to_owned(), 
            d_type: d_t
        };
        return data_value
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
            match unparsed_token.parse::<f64>() // temp thing to only save numbers 
            {
                Ok(ok) => determined_tokens.push(Token::Data(DataValue::new(unparsed_token))),
                Err(e) => panic!("Not number")
            }
        }
    }
    return determined_tokens
}