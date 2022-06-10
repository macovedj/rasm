#![warn(rust_2018_idioms)]
#![feature(iter_advance_by)]
use std::fs;
use std::str;
use regex::Regex;
mod tokens;
mod parser;
mod ast;
mod compiler;

fn main() {
    let contents = fs::read_to_string("./add4.wat")
        .expect("Something went wrong reading the file");
  
    let oneline: String = str::replace(&contents, "\n", "");
    let re = Regex::new(r"\s+").unwrap();
    let text = re.replace_all(&oneline, " ");
    let chars = text.trim();
    let parsed = parser::parse(&chars);
    let ast = ast::ast_builder(parsed);

    let bytes = compiler::compiler(ast);
    println!("BYTES {:?}", bytes);
    let hex = bytes.iter().map(|x| format!("{:#04x}", x)).collect::<Vec<_>>();


    println!("HEX {:?}", hex);
}
