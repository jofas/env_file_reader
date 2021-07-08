use regex::Regex;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(
  path: P,
) -> Result<HashMap<String, String>, std::io::Error> {
  let content = fs::read_to_string(path)?;

  let variable = Regex::new(
    r#"^(export )?(?P<key>[0-9A-Za-z_\-]+)="?(?P<val>[^"]*)"?$"#,
  )
  .unwrap();

  let mut config = HashMap::new();

  for line in content.lines() {
    if let Some(c) = variable.captures(line) {
      config.insert(
        c.name("key").unwrap().as_str().to_owned(),
        c.name("val").unwrap().as_str().to_owned(),
      );
    }
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
