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
pub struct Func {
  pub export: String,
  pub params: Vec<WasmPrimitives>,
  pub body: Vec<String>,
  pub result: WasmPrimitives
}

impl Func {
  fn add_param(&mut self, param: WasmPrimitives) {
    self.params.push(param);
  }

  fn add_to_body(&mut self, instr: String) {
    self.body.push(instr);
  }

  fn modify_result(&mut self, res: WasmPrimitives) {
    self.result = res;
  }
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
pub struct WasmModule {
  pub funcs: Vec<Func>
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
pub struct Ast {
  pub mods: Vec<WasmModule>
}

pub fn ast_builder(tokens: impl Iterator<Item = parser::Token>) -> Ast {
  let tokens: Vec<_> = tokens.collect();
  let mut ast: Ast = Ast { mods: Vec::new()};
  let mut cur_module: usize = 0;
  let mut index = 0;
  while index < tokens.len() {
    match tokens[index].kind {
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
          export: String::from(""),
          params: Vec::new(),
          body: Vec::new(),
          result: WasmPrimitives::NULL
        };
        index += 1;
        while func_def {
          match tokens[index].kind {
            tokens::TokenTypes::EXPORT => {
              let ref export = tokens[index + 1].value;
              cur_func.export = String::from(export);
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
              while matches!(tokens[index + 1].kind, tokens::TokenTypes::PARAM) {
                if tokens[index + 1].value == "i32" {
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
              while !matches!(tokens[index].kind, tokens::TokenTypes::RPAR) {
                let ref instr = tokens[index].value;
                cur_func.add_to_body(String::from(instr));
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