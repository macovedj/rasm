use std::str;
use super::tokens;


#[derive(Debug)]
#[derive(Clone)]
pub struct Token<'a> {
  pub kind: tokens::TokenTypes,
  pub value: &'a str
}

pub fn parse(chars: &str) -> Vec::<Token<'_>> {
  let mut index = 0;
  let mut tokens = Vec::new();
  loop {
    if index == chars.len() - 1 {
      break;
    }
    match &chars[index..index + 1] {
      "(" => {
        tokens.push(Token {kind: tokens::TokenTypes::LPAR, value: "("});
        index += 1;
      }
      ")" => {
        tokens.push(Token {kind: tokens::TokenTypes::RPAR, value: ")"});
        index += 1;
      }
      " " => {
        index += 1;
      }
      _ => {
        let mut tok_length = 1;
        let mut scanning = true;
        while (scanning) {
          match &chars[index..index + tok_length] {
            "module" => {
              tokens.push(Token {kind: tokens::TokenTypes::MOD, value: "module"});
              index += tok_length;
              scanning = false;
              tok_length = 1;
            }
            "func" => {
              tokens.push(Token {kind: tokens::TokenTypes::FUNC, value: "func"});
              index += tok_length;
              scanning = false;
              tok_length = 1;
            }
            "export" => {
              tokens.push(Token {kind: tokens::TokenTypes::EXPORT, value: "export"});
              index += tok_length;
              scanning = false;
              tok_length = 1;
            }
            "param" => {
              tokens.push(Token {kind: tokens::TokenTypes::PARAMDECL, value: "param"});
              index += tok_length;
              scanning = false;
              tok_length = 1;
            }
            "i32" => {
              if &chars[index + tok_length..index + tok_length + 1] == "." {
                tok_length += 1;
                continue;
              }
              tokens.push(Token {kind: tokens::TokenTypes::PARAM, value: "i32"});
              index += tok_length;
              scanning = false;
              tok_length = 1;
            }
            "result" => {
              tokens.push(Token {kind: tokens::TokenTypes::RESULT, value: "result"});
              index += tok_length;
              scanning = false;
              tok_length = 1;
            }
            "local.get" => {
              tokens.push(Token {kind: tokens::TokenTypes::LOCAL_GET, value: "local.get"});
              index += tok_length;
              scanning = false;
              tok_length = 1;
            }
            "i32.add" => {
              tokens.push(Token {kind: tokens::TokenTypes::ADD_I32, value: "i32.add"});
              index += tok_length;
              scanning = false;
              tok_length = 1;
            }
            _ => {
              if &chars[index + tok_length..index + tok_length + 1] == ")" || &chars[index + tok_length..index + tok_length + 1] == " " {
                tokens.push(Token {kind: tokens::TokenTypes::LITERAL, value: &chars[index..index + tok_length]});
                index += tok_length;
                scanning = false;
                tok_length = 1;
              }
              tok_length += 1;
            }
          }
        }
      }
    }
  }
  
  return tokens
}