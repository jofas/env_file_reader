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
