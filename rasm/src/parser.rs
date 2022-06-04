use std::fs;
use std::str;
use super::tokens;
use regex::Regex;

#[derive(Debug)]
pub struct Token {
  kind: tokens::tokenTypes,
  value: String
}

pub fn parse(file: &str) -> Vec<Token> {
  let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
  
  let oneline: String = str::replace(&contents, "\n", "");
  let re = Regex::new(r"\s+").unwrap();
  let text = re.replace_all(&oneline, " ");
  let chars = text.chars();
  let mut cur_token = String::new();
  let mut tokens: Vec<Token> = Vec::new();
  for tok in chars {
    println!("TOKEN: {}",tok);
    match tok {
      '(' => {
        tokens.push(Token {kind: tokens::tokenTypes::LPAR, value: String::from("(")});
      }
      ')' => {
        if cur_token == "module" {
          tokens.push(Token {kind: tokens::tokenTypes::MOD, value: String::from("module")}); 
        } else if cur_token == "func" {
          tokens.push(Token {kind: tokens::tokenTypes::FUNC, value: String::from("func")}); 
        } else if cur_token == "export" {
          tokens.push(Token {kind: tokens::tokenTypes::EXPORT, value: String::from("export")}); 
        } else if cur_token == "param" {
          tokens.push(Token {kind: tokens::tokenTypes::PARAMDECL, value: String::from("param")}); 
        } else if cur_token == "i32" {
          tokens.push(Token {kind: tokens::tokenTypes::PARAM, value: String::from("i32")}); 
        } else if cur_token == "result" {
          tokens.push(Token {kind: tokens::tokenTypes::RESULT, value: String::from("result")}); 
        } else if cur_token == "local.get" {
          tokens.push(Token {kind: tokens::tokenTypes::LOCAL_GET, value: String::from("local.get")}); 
        } else if cur_token == "i32.add" {
          tokens.push(Token {kind: tokens::tokenTypes::ADD_I32, value: String::from("i32.add")}); 
        } else {
          tokens.push(Token {kind: tokens::tokenTypes::LITERAL, value: cur_token });
        }
        cur_token = String::new();
        tokens.push(Token {kind: tokens::tokenTypes::RPAR, value: String::from(')') });

      }
      ' ' => {
        if cur_token == "module" {
          tokens.push(Token {kind: tokens::tokenTypes::MOD, value: String::from("module")}); 
        } else if cur_token == "func" {
          tokens.push(Token {kind: tokens::tokenTypes::FUNC, value: String::from("func")}); 
        } else if cur_token == "export" {
          tokens.push(Token {kind: tokens::tokenTypes::EXPORT, value: String::from("export")}); 
        } else if cur_token == "param" {
          tokens.push(Token {kind: tokens::tokenTypes::PARAMDECL, value: String::from("param")}); 
        } else if cur_token == "i32" {
          tokens.push(Token {kind: tokens::tokenTypes::PARAM, value: String::from("i32")}); 
        } else if cur_token == "result" {
          tokens.push(Token {kind: tokens::tokenTypes::RESULT, value: String::from("result")}); 
        } else if cur_token == "local.get" {
          tokens.push(Token {kind: tokens::tokenTypes::LOCAL_GET, value: String::from("local.get")}); 
        } else if cur_token == "i32.add" {
          tokens.push(Token {kind: tokens::tokenTypes::ADD_I32, value: String::from("i32.add")}); 
        } else {
          tokens.push(Token {kind: tokens::tokenTypes::LITERAL, value: cur_token });
        }
        cur_token = String::new();
      }
      _ => {
        cur_token.push(tok);
      }
    }
  }

  return tokens
}

// const fs = require('fs');
// const { tokenTypes } = require('./tokens')

// const parse = async (file) => { 
//   const tokens = text.reduce((acc, cur) => {
//     let {curToken, tokenized} = acc
//     switch(cur) {
//       case "(":
//         tokenized.push({ type: tokenTypes.LPAR, value: "("})
//         return {curToken: [], tokenized}
//       case ")":
//         switch(curToken.join('')) {
//           case "module":
//             tokenized.push({type: tokenTypes.MOD, value: "module"})
//             break;
//           case "data":
//             tokenized.push({type: tokenTypes.DATA, value: "data"})
//             break;
//           case "func":
//             tokenized.push({type: tokenTypes.FUNC, value: "func"})
//             break;
//           case "export":
//             tokenized.push({type: tokenTypes.EXPORT, value: "export"})
//             break;
//           case "param":
//             tokenized.push({type: tokenTypes.PARAMDECL, value: "param"})
//             break;
//           case "i32":
//             tokenized.push({type: tokenTypes.PARAM, value: "i32"})
//             break;
//           case "result":
//             tokenized.push({type: tokenTypes.RESULT, value: "result"})
//             break;
//           case "local.get":
//             tokenized.push({type: tokenTypes.LOCAL_GET, value: "local.get"})
//             break;
//           case "i32.add":
//             tokenized.push({type: tokenTypes.ADD_I32, value: "i32.add"})
//             break;
//           case "i32.load8_u":
//             tokenized.push({type: tokenTypes.LOAD_U8, value: "i32.load8_u"})
//             break;
//           default:
//             if (curToken.join('') !== '') {
//               tokenized.push({type: tokenTypes.LITERAL, value: curToken.join('')})
//             }
//             break;
//         }
//         tokenized.push({ type: tokenTypes.RPAR, value: ")"})
//         return {curToken: [], tokenized}
//       case " ":
//         switch(curToken.join('')) {
//           case "module":
//             tokenized.push({type: tokenTypes.MOD, value: "module"})
//             return {curToken: [], tokenized}
//           case "data":
//             tokenized.push({type: tokenTypes.DATA, value: "data"})
//             return {curToken: [], tokenized}
//           case "func":
//             tokenized.push({type: tokenTypes.FUNC, value: "func"})
//             return {curToken: [], tokenized}
//           case "export":
//             tokenized.push({type: tokenTypes.EXPORT, value: "export"})
//             return {curToken: [], tokenized}
//           case "param":
//             tokenized.push({type: tokenTypes.PARAMDECL, value: "param"})
//             return {curToken: [], tokenized}
//           case "i32":
//             tokenized.push({type: tokenTypes.PARAM, value: "i32"})
//             return {curToken: [], tokenized}
//           case "result":
//             tokenized.push({type: tokenTypes.RESULT, value: "result"})
//             return {curToken: [], tokenized}
//           case "local.get":
//             tokenized.push({type: tokenTypes.LOCAL_GET, value: "local.get"})
//             return {curToken: [], tokenized}
//           case "i32.add":
//             tokenized.push({type: tokenTypes.ADD_I32, value: "i32.add"})
//             return {curToken: [], tokenized}
//           case "i32.load8_u":
//             tokenized.push({type: tokenTypes.LOAD_U8, value: "i32.load8_u"})
//             return {curToken: [], tokenized}
//           case "":
//             return {curToken: [], tokenized}
//           default:
//             tokenized.push({type: tokenTypes.LITERAL, value: curToken})
//             return {curToken: [], tokenized}
//         }
//       default:
//         curToken.push(cur)
//         return acc
//     }
//   }, {curToken: [], tokenized: []})
//   return tokens.tokenized
// }

// module.exports = {parse}