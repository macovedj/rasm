use super::ast;

#[derive(PartialEq, Eq)]
struct Sig<'a> {
  params: &'a [ast::WasmPrimitives],
  result: ast::WasmPrimitives,
}

impl <'a> Sig<'a> {
  pub fn new(func: &'a ast::Func) -> Self {
    Sig { params: &func.params[..], result: func.result }
  }

  pub fn write(&self, bytes: &mut Vec<u8>) {
    bytes.push(0x60);
    bytes.push(self.params.len() as u8);
    for param in &self.params[..] {
      match param {
        ast::WasmPrimitives::i32 => {
          bytes.push(0x7f);
        }
        _ => {
          continue;
        }
      }
    }
    // number of return values
    bytes.push(0x01);
    match &self.result {
      ast::WasmPrimitives::i32 => {
        bytes.push(0x7f);
      }
      _ => {}
    } 
  }
}

pub fn compiler(ast: ast::Ast) -> Vec<u8> {
  let mut bytes = Vec::new();
  bytes.push(0x00);
  bytes.push(0x61);
  bytes.push(0x73);
  bytes.push(0x6d);
  bytes.push(0x01);
  bytes.push(0x00);
  bytes.push(0x00);
  bytes.push(0x00);
  bytes.push(0x01);
  let funcs = &ast.mods[0].funcs[..];
  let mut sigs = Vec::new();
  for func in funcs {
    sigs.push(Sig::new(func));
  }
  let num_of_types = sigs.len();
  let type_section_size: u8 = (&sigs[..].into_iter()
  .fold(0, |acc, cur| acc + 4 + cur.params.len()) + 1).try_into().unwrap();
  bytes.push(type_section_size);
  bytes.push(num_of_types as u8);
  for sig in &sigs {
    sig.write(&mut bytes);
  }
  bytes.push(0x03);
  let num_of_funcs = funcs.len() as u8;
  bytes.push(num_of_funcs + 1);
  bytes.push(num_of_funcs);
  for func in funcs {
    for (i, sig) in sigs.iter().enumerate() {
      if sig == &Sig::new(&func) {
        bytes.push(i as u8);
      }
    }
  }

  let num_of_exports = funcs.into_iter()
    .filter(|func| func.export.len() > 0)
    .fold(0, |acc: u8, _| acc + 1);
  
  if num_of_exports > 0 {
    let mut i = 0;
    bytes.push(0x07);
    let export_names = funcs.iter().map(|f| &f.export).collect::<Vec<_>>();
    let hex_size = funcs.iter().map(|f| &f.export).fold(0, |acc, cur| acc + cur.len() + 3);
    bytes.push((hex_size + 1).try_into().unwrap());
    bytes.push(num_of_exports.try_into().unwrap());
    for export in export_names {
      if export.len() == 0 {
        continue;
      }
      bytes.push(export.len().try_into().unwrap());
      for char in export.bytes() {
        if char >= 97 {
          bytes.push(char.to_ascii_lowercase());
        } else {
          bytes.push(char.to_ascii_uppercase());
        }
      }
      bytes.push(0x00);
      bytes.push(i.try_into().unwrap());
      i += 1;
    }
  }

  bytes.push(0x0a);
  let body_length = funcs.iter().map(|f| &f.body)
  .fold(0,|all_length, cur_body| all_length + cur_body.len() + 3) + 1;
  
  bytes.push(body_length.try_into().unwrap());
  bytes.push(funcs.len().try_into().unwrap());

  let bodies = funcs.iter().map(|f| &f.body).collect::<Vec<_>>();
  // println!()
  for body in bodies {
    bytes.push((body.len() + 2).try_into().unwrap());
    bytes.push(0x00);
    for instr in body {
      match instr.as_str() {
        "local.get" => {
          bytes.push(0x20);
        }
        "i32.add" => {
          bytes.push(0x6a);
        }
        _ => {
          for byte in instr.chars() {
            bytes.push(String::from(byte).parse::<u8>().unwrap());
          }
        }
      }
    }
    bytes.push(0x0b);
  }

  return bytes
}