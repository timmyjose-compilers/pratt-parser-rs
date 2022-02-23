use std::error::Error;

pub mod ast;
pub mod evaluator;
pub mod parser;
pub mod scanner;
pub mod token;

pub type GenError = Box<dyn Error>;
