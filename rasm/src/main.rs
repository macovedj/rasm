mod tokens;
mod parser;

fn main() {
    println!("Hello, world!");
    println!("In file {}", "./add4.wat");

    parser::parse("./add4.wat");

    println!("TOKENS{:?}", tokens::tokenTypes::LPAR);
}
