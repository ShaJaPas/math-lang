use ast::Node;
use parser::Rule;

pub mod ast;
pub mod interpreter;
pub mod operations;
pub mod parser;
pub mod vm;

pub trait Compile {
    type Output;

    fn from_ast(ast: Node) -> Self::Output;

    fn from_source(source: &str) -> Result<Self::Output, Box<pest::error::Error<Rule>>> {
        Ok(Self::from_ast(parser::parse(source)?))
    }
}
