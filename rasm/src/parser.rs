use std::str;
use super::tokens;


#[derive(Debug, Clone)]
pub struct Token {
  pub kind: tokens::TokenTypes,
  pub value: String
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
  chars: &'a str,
  index: usize
}

impl<'a> Parser<'a> {
  pub fn new(chars: &'a str) -> Self {
    Parser { chars, index: 0 }
  }
}

impl<'a> Iterator for Parser<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
      if self.index == self.chars.len() - 1 {
        return None;
      }
      match &self.chars[self.index..self.index + 1] {
        "(" => {
          self.index += 1;
          return Some(Token {kind: tokens::TokenTypes::LPAR, value: String::from("(")});
        }
        ")" => {
          self.index += 1;
          return Some(Token {kind: tokens::TokenTypes::RPAR, value: String::from(")")});
        }
        " " => {
          self.index += 1;
          return self.next();
        }
        _ => {
          let mut tok_length = 1;
          loop {
            match &self.chars[self.index..self.index + tok_length] {
              "module" => {
                self.index += tok_length;
                break Some(Token {kind: tokens::TokenTypes::MOD, value: String::from("module")});
              }
              "func" => {
                self.index += tok_length;
                break Some(Token {kind: tokens::TokenTypes::FUNC, value: String::from("func")});
              }
              "export" => {
                self.index += tok_length;
                break Some(Token {kind: tokens::TokenTypes::EXPORT, value: String::from("export")});
              }
              "param" => {
                self.index += tok_length;
                break Some(Token {kind: tokens::TokenTypes::PARAMDECL, value: String::from("param")});
              }
              "i32" => {
                if &self.chars[self.index + tok_length..self.index + tok_length + 1] == "." {
                  tok_length += 1;
                  continue;
                }
                self.index += tok_length;
                break Some(Token {kind: tokens::TokenTypes::PARAM, value: String::from("i32")});
              }
              "result" => {
                self.index += tok_length;
                break Some(Token {kind: tokens::TokenTypes::RESULT, value: String::from("result")});
              }
              "local.get" => {
                self.index += tok_length;
                break Some(Token {kind: tokens::TokenTypes::LOCAL_GET, value: String::from("local.get")});
              }
              "i32.add" => {
                self.index += tok_length;
                break Some(Token {kind: tokens::TokenTypes::ADD_I32, value: String::from("i32.add")});
              }
              _ => {
                if &self.chars[self.index + tok_length..self.index + tok_length + 1] == ")" || &self.chars[self.index + tok_length..self.index + tok_length + 1] == " " {
                  let tok = Some(Token {kind: tokens::TokenTypes::LITERAL, value: String::from(&self.chars[self.index..self.index + tok_length])
                    .replace(&['\\', '"'], "")
                  });
                  self.index += tok_length;
                  break tok;
                }
                tok_length += 1;
              }
            }
          }
        }
      }
    }
}