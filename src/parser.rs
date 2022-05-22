// Parse lexicals (?)
// make AST

use crate::lexer::{self, Token, DataValue};
use crate::reconized_symbols;

struct AST {
    value: Token,
    left: Option<Box<AST>>,
    right: Option<Box<AST>>
}




