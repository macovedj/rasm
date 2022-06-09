#[derive(Debug)]
#[derive(Clone)]
pub enum TokenTypes {
  LPAR,
  RPAR,
  MOD,
  FUNC,
  EXPORT,
  PARAMDECL,
  PARAM,
  RESULT,
  LITERAL,
  LOCAL_GET,
  ADD_I32,
  DATA,
  LOAD_U8
}