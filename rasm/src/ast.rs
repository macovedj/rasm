use super::parser;
use super::tokens;

// #[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Clone, Debug)]
pub enum WasmPrimitives {
  i32,
  i64,
  f32,
  f64,
  NULL
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Clone, Debug)]
pub struct Func<'a> {
  pub export: &'a str,
  pub params: Vec<WasmPrimitives>,
  pub body: Vec<&'a str>,
  pub result: WasmPrimitives
}

impl<'a> Func<'a> {
  fn add_param(&mut self, param: WasmPrimitives) {
    self.params.push(param);
  }

  fn add_to_body(&mut self, instr: &'a str) {
    self.body.push(instr);
  }

  fn modify_result(&mut self, res: WasmPrimitives) {
    self.result = res;
  }
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
pub struct WasmModule<'a> {
  pub funcs: Vec<Func<'a>>
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
pub struct Ast<'a> {
  pub mods: Vec<WasmModule<'a>>
}

pub fn ast_builder(tokens: Vec<parser::Token<'_>>) -> Ast {
  let tok_slice = &tokens[..];
  let mut ast = Ast { mods: Vec::new()};
  let mut cur_module: usize = 0;
  let mut index = 0;
  while (index < tok_slice.len()) {
    match(tok_slice[index].kind) {
      tokens::TokenTypes::LPAR => {
        index += 1;
        continue;
      }
      tokens::TokenTypes::RPAR => {
        index += 1;
        continue;
      }
      tokens::TokenTypes::MOD => {
        cur_module = ast.mods.len();
        ast.mods.push(WasmModule {funcs: Vec::new()});
        index += 1;
      }
      tokens::TokenTypes::FUNC => {
        let mut func_def: bool;
        let mut cur_func: Func;
        func_def = true;
        cur_func = Func {
          export: "",
          params: Vec::new(),
          body: Vec::new(),
          result: WasmPrimitives::NULL
        };
        index += 1;
        while func_def {
          match tok_slice[index].kind {
            tokens::TokenTypes::EXPORT => {
              cur_func.export = tok_slice[index + 1].value;
              index += 2;
            }
            tokens::TokenTypes::RPAR => {
              index += 1;
              continue;
            }
            tokens::TokenTypes::LPAR => {
              index += 1;
              continue;
            }
            tokens::TokenTypes::PARAMDECL => {
              while matches!(tok_slice[index + 1].kind, tokens::TokenTypes::PARAM) {
                if tok_slice[index + 1].value == "i32" {
                  cur_func.add_param(WasmPrimitives::i32);
                  index += 1;
                } else {
                  index += 1;
                  continue;
                }
              }
              index += 2;
            }
            tokens::TokenTypes::RESULT => {
              cur_func.modify_result(WasmPrimitives::i32);
              index += 3;
            }
            _ => {
              while !matches!(tok_slice[index].kind, tokens::TokenTypes::RPAR) {
                cur_func.add_to_body(&tok_slice[index].value);
                index += 1;
              }
             index += 1;
              func_def = false;
              ast.mods[cur_module].funcs.push(cur_func);
              break;
            }
          }
        }
      }
      _ => {

      }
    }
  }
  
  return ast
}