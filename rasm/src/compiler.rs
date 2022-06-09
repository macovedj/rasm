use super::ast;

pub fn compiler(ast: ast::Ast) -> String {
  let bytes = String::from("0061736d0100000001");
  let numOfSections = ast.mods[0].funcs.len();
  println!("NUMBER OF SECTIONS {}", numOfSections);
  return String::from("bytes")
}