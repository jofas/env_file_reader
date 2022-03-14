#![doc = include_str!("../README.md")]

use regex::Regex;

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

lazy_static! {
  static ref ENV_VARIABLE: Regex = Regex::new(
    r#"^(export )?(?P<key>[0-9A-Za-z_\-]+)="?(?P<val>[^"]*)"?$"#,
  )
  .unwrap();
}

pub fn read_file<P: AsRef<Path>>(
  path: P,
) -> Result<HashMap<String, String>, std::io::Error> {
  let content = fs::read_to_string(path)?;

  let mut config = HashMap::new();

  let mut line = String::new();

  for new_line in content.lines() {
    let new_line = new_line.trim_start();

    if new_line.ends_with("\\") {
      line = format!("{}{}", line, &new_line[0..new_line.len() - 1]);
      continue;
    }

    line = format!("{}{}", line, new_line);

    if let Some(c) = ENV_VARIABLE.captures(&line) {
      config.insert(
        c.name("key").unwrap().as_str().to_owned(),
        c.name("val").unwrap().as_str().to_owned(),
      );
    }

    line = String::new();
  }

  Ok(config)
}

pub fn read_files<P: AsRef<Path>>(
  paths: &[P],
) -> Result<HashMap<String, String>, std::io::Error> {
  let mut res = HashMap::new();
  for path in paths {
    let map = read_file(path)?;
    map.into_iter().for_each(|(k, v)| {
      res.insert(k, v);
    });
  }
  Ok(res)
}
