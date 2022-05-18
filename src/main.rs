/*

Notes:
Lexer and Parser:
    - Lexer: Module to parse a "line" of script into different tokens (datatype, variable, keyword, value, operators, etc)
    - Parser: Processes an array of "tokens" that the lexer has classified and process the "grammar" of the line
Process data:
   - make the code, do the thing (idk how to do that yet)

Order:
1) parse and have rust "understand" tokens
2) calculator
3) ???
4) scripting language
 */

mod lexer;
pub mod reconized_symbols;
use std::fmt::Result;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::env::args;
use std::io::BufReader;
use std::vec::Vec;

fn main() {
   println!("Hello, World!");
   let file_contents = match read_file("a.txt") {
      Ok(ok) => ok,
      Err(e) => panic!("An error occured with reading the file. Error: {}", e)
   };

   let mut lexed_file: Vec<Vec<lexer::Token>> = vec![];
   for line in &file_contents
   {
      lexed_file.push(lexer::determine_tokens(&line));
   }
   println!("{:?}", file_contents);
   println!("{:?}", lexed_file);
}

fn read_file(filepath: &str) -> io::Result<Vec<String>>
{
   let f = File::open(filepath)?;
   let split_iter = BufReader::new(f).lines();
   let mut lines_of_code = vec![];
   for i in split_iter
   {
      lines_of_code.push(i?.to_owned());
   }
   return Ok(lines_of_code)
}