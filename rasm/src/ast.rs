use super::parser;
use super::tokens;

// #[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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

pub fn ast_builder(mut tokens: impl Iterator<Item = parser::Token>) -> Ast {
  let mut ast: Ast = Ast { mods: Vec::new() };
  let mut cur_module: usize = 0;
  while let Some(mut token) = tokens.next() {
    match token.kind {
      tokens::TokenTypes::LPAR => {
        continue;
      }
      tokens::TokenTypes::RPAR => {
        continue;
      }
      tokens::TokenTypes::MOD => {
        cur_module = ast.mods.len();
        ast.mods.push(WasmModule {funcs: Vec::new()});
        continue;
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
        tokens.next();
        if let Some(potential_token) = tokens.next() {
          token = potential_token;
        };
        while func_def {
          match token.kind {
            tokens::TokenTypes::EXPORT => {
              if let Some(next_token) = tokens.next() {
                let export = &next_token.value;
                cur_func.export = String::from(export);
                if let Some(potential_token) = tokens.next() {
                  token = potential_token;
                };
              };
              continue;
            }
            tokens::TokenTypes::RPAR => {
              if let Some(potential_token) = tokens.next() {
                token = potential_token;
              };
              continue;
            }
            tokens::TokenTypes::LPAR => {
              if let Some(potential_token) = tokens.next() {
                token = potential_token;
              };
              continue;
            }
            tokens::TokenTypes::PARAMDECL => {
              if let Some(mut next_token) = tokens.next() {
                while matches!(next_token.kind, tokens::TokenTypes::PARAM) {
                  if next_token.value == "i32" {
                    cur_func.add_param(WasmPrimitives::i32);
                  }
                  if let Some(potential_token) = tokens.next() {
                    next_token = potential_token;
                  } 
                  continue;
                }
                if let Some(potential_token) = tokens.next() {
                  token = potential_token;
                };
                continue;
              };
            }
            tokens::TokenTypes::RESULT => {
              cur_func.modify_result(WasmPrimitives::i32);
              tokens.next();
              tokens.next();
              if let Some(potential_token) = tokens.next() {
                token = potential_token;
              };
              continue;
            }
            _ => {
              while !matches!(token.kind, tokens::TokenTypes::RPAR) {
                let ref instr = token.value;
                cur_func.add_to_body(String::from(instr));
                if let Some(potential_token) = tokens.next() {
                  token = potential_token;
                };
              }
              tokens.next();
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