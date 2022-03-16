#![doc = include_str!("../README.md")]

use lalrpop_util::lalrpop_mod;

use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

lalrpop_mod!(env_file);

mod lexer;

pub use lexer::ParseError;

pub fn read_str(s: &str) -> Result<HashMap<String, String>, Error> {
  env_file::EnvFileParser::new()
    .parse(lexer::Lexer::new(s))
    .map_err(|_| Error::new(ErrorKind::InvalidInput, &ParseError))
}

pub fn read_file<P: AsRef<Path>>(
  path: P,
) -> Result<HashMap<String, String>, Error> {
  let content = fs::read_to_string(path)?;

  read_str(&content)
}

pub fn read_files<P: AsRef<Path>>(
  paths: &[P],
) -> Result<HashMap<String, String>, Error> {
  let mut res = HashMap::new();

  for path in paths {
    res.extend(read_file(path)?);
  }

  Ok(res)
}

#[cfg(test)]
mod test {
  use super::read_str;

  #[test]
  fn wrong_idents() {
    let s = "key_with#=x";
    assert!(read_str(s).is_err());

    let s = "key_with`quotes`=x";
    assert!(read_str(s).is_err());

    let s = "key_with=do-not-work=x";
    assert!(read_str(s).is_err());

    let s = "key with whitespace=x";
    assert!(read_str(s).is_err());
  }

  #[test]
  fn comments() {
    let s = "
      # a comment
      key=val # a comment at the end of the line
    ";

    let m = read_str(s).unwrap();

    assert_eq!(&m["key"], "val");
  }

  #[test]
  fn empty_value() {
    let s = "
      key1=
      key2=something
      key3=";

    let m = read_str(s).unwrap();

    assert_eq!(&m["key1"], "");
    assert_eq!(&m["key2"], "something");
    assert_eq!(&m["key3"], "");
  }
}
