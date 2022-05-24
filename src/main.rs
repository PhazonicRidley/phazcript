/*
Notes and overall TODO:
Support seperators, quotes and parentheses
Create keyword varient in Token enum
Support variables.
    - syntax: int variable = 3
    - string my_string = "my string"
    - bool binary = true
    - use a map to relate variable names to their literals

Support functions
    - syntax: return_type function_name(int args, string more_args) {}
    - support scopes. other maps maybe?
Support conditionals and loops (these will be internal functions)
 */

mod lexer;
mod parser;
pub mod reconized_symbols;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    vec::Vec,
};

fn main() {
    let file_contents = match read_file("a.txt") {
        Ok(ok) => ok,
        Err(e) => panic!("An error occured with reading the file.\nError: {}", e),
    };

    let mut lexed_file: Vec<Vec<lexer::Token>> = vec![];
    for line in &file_contents {
        lexed_file.push(lexer::determine_tokens(&line));
    }
    //println!("{:?}", lexed_file);
    parser::test_tree(&lexed_file[0]);
}

/**
 * Reads a file. Takes a filepath as a str
 * Returns an IO result of a vector of each line in the file.
 */
fn read_file(filepath: &str) -> io::Result<Vec<String>> {
    let f = File::open(filepath)?;
    let split_iter = BufReader::new(f).lines();
    let mut lines_of_code = vec![];
    for i in split_iter {
        lines_of_code.push(i?.to_owned());
    }
    return Ok(lines_of_code);
}
