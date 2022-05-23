// Parse lexicals (?)
// make AST

use std::fmt::Error;
use crate::lexer::{Token, Type};

#[derive(Debug)]
struct AST {
    value: Token,
    left: Option<Box<AST>>,
    right: Option<Box<AST>>
}

impl AST {
    fn new(line: &Vec<Token>) -> Result<AST, Error>  {
        // seperate out operators
        if line.len() == 1 {
            return Ok(AST {value: line[0].clone(), left: None, right: None})
        }
        for i in 0..line.len() {
            if let Token::Operator(_op) = line[i].clone()
            {
                return Ok(AST {
                    value: line[i].clone(), 
                    left: Some(Box::new(AST::new(&line[0..i].to_vec())?)), 
                    right: Some(Box::new(AST::new(&line[i+1..line.len()].to_vec())?))
                })
                
            }
        }
       return Err(Error) // should not trigger
    }

    fn evaluate(self) -> Result<Type, Error>
    {
        if self.left.is_none() && self.right.is_none()
        {
            if let Token::Data(data) = self.value
            {
                return Ok(data.value);
            }
            else {
                return Err(Error);
            }
        }

        if let Token::Operator(op) = self.value
        {
            let ans = match op.as_str()
            {
                "+" => self.left.expect("error on left").evaluate()? + self.right.expect("error on right").evaluate()?,
                "-" => self.left.expect("error on left").evaluate()? - self.right.expect("error on right").evaluate()?,
                _ => panic!("Not implemented yet")
            };
            return Ok(ans)
        }
        return Err(Error)
        
        

    }
}


pub fn test_tree(line: &Vec<Token>)
{
    let tree = AST::new(line).unwrap();
    
    //println!("{:?}", tree);
    let res = tree.evaluate().unwrap();
    println!("{:?}", res);
}
