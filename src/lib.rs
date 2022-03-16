#![doc = include_str!("../README.md")]

use lalrpop_util::lalrpop_mod;

use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

lalrpop_mod!(env_file);

mod lexer;

pub use lexer::ParseError;

/// Parses an environment file that is passed as a `&str`.
///
/// Returns an error if if the string is ill-formatted.
/// Read the [crate's](crate) documentation for information about the
/// format that `env-file-reader` supports.
///
/// **Example:**
///
/// ```rust
/// use env_file_reader::read_str;
///
/// const ENV_FILE: &str = "
///   CLIENT_ID=YOUR_CLIENT_ID
///   CLIENT_SECRET=YOUR_CLIENT_SECRET
/// ";
///
/// fn main() -> std::io::Result<()> {
///   let env_variables = read_str(ENV_FILE)?;
///
///   assert_eq!(&env_variables["CLIENT_ID"], "YOUR_CLIENT_ID");
///   assert_eq!(&env_variables["CLIENT_SECRET"], "YOUR_CLIENT_SECRET");
///
///   Ok(())
/// }
/// ```
///
pub fn read_str(s: &str) -> Result<HashMap<String, String>, Error> {
  env_file::EnvFileParser::new()
    .parse(lexer::Lexer::new(s))
    .map_err(|_| Error::new(ErrorKind::InvalidInput, &ParseError))
}

/// Parses the environment file at the specified `path`.
///
/// Returns an error if reading the file was unsuccessful or if the
/// file is ill-formatted.
/// Read the [crate's](crate) documentation for information about the
/// format that `env-file-reader` supports.
///
/// **Example:**
///
/// `examples/.env`:
///
/// ```ini
/// CLIENT_ID=YOUR_CLIENT_ID
/// CLIENT_SECRET=YOUR_CLIENT_SECRET
/// ```
///
/// ```rust
/// use env_file_reader::read_file;
///
/// fn main() -> std::io::Result<()> {
///   let env_variables = read_file("examples/.env")?;
///
///   assert_eq!(&env_variables["CLIENT_ID"], "YOUR_CLIENT_ID");
///   assert_eq!(&env_variables["CLIENT_SECRET"], "YOUR_CLIENT_SECRET");
///
///   Ok(())
/// }
/// ```
///
pub fn read_file<P: AsRef<Path>>(
  path: P,
) -> Result<HashMap<String, String>, Error> {
  let content = fs::read_to_string(path)?;

  read_str(&content)
}

/// Parses multiple environment files at the specified `paths`
/// and constructs a single [HashMap] with the merged environment
/// variables from the files.
///
/// This is a vectorized version of [read_file], where a single
/// output is constructed from calling [read_file] for each file
/// provided as an argument to this function.
/// The order in which the `paths` are defined is maintained, so
/// if an environment file with a higher index exposes the same
/// environment variable as a file with a lower index, the value of
/// the file with the higher index is exposed by the returned
/// [HashMap].
///
/// Returns an error if reading one file was unsuccessful or if one
/// file is ill-formatted.
/// Read the [crate's](crate) documentation for information about the
/// format that `env-file-reader` supports.
///
/// **Example:**
///
/// `examples/.env`:
///
/// ```ini
/// CLIENT_ID=YOUR_CLIENT_ID
/// CLIENT_SECRET=YOUR_CLIENT_SECRET
/// ```
///
/// `examples/.env.utf8`:
///
/// ```ini
/// 🦄=💖
/// 💖=🦄
/// ```
///
/// ```rust
/// use env_file_reader::read_files;
///
/// fn main() -> std::io::Result<()> {
///   let env_variables = read_files(&[
///     "examples/.env",
///     "examples/.env.utf8",
///   ])?;
///
///   assert_eq!(&env_variables["CLIENT_ID"], "YOUR_CLIENT_ID");
///   assert_eq!(&env_variables["CLIENT_SECRET"], "YOUR_CLIENT_SECRET");
///   assert_eq!(&env_variables["🦄"], "💖");
///   assert_eq!(&env_variables["💖"], "🦄");
///
///   Ok(())
/// }
/// ```
///
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

  #[test]
  fn empty_string() {
    let m = read_str("").unwrap();
    assert_eq!(m.len(), 0);
  }
}
