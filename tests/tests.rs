use env_file_reader::{read_file, read_files};

#[test]
fn test_read_file() {
  let map = read_file("examples/env_file1").unwrap();

  assert_eq!(map.get("a"), Some(&"C".to_owned()));
  assert_eq!(map.get("b"), Some(&"C".to_owned()));
  assert_eq!(
    map.get("some_longer_name"),
    Some(
      &"hello I can do long texts too, with whitespaces!".to_owned()
    )
  );
  assert_eq!(map.get("export"), Some(&"WORKS".to_owned()));
  assert_eq!(map.get("line_break"), Some(&"\\".to_owned()));
}

#[test]
fn test_read_files() {
  let files = vec!["examples/env_file1", "examples/env_file2"];
  let map = read_files(&files).unwrap();

  assert_eq!(map.get("a"), Some(&"D".to_owned()));
  assert_eq!(map.get("b"), Some(&"C".to_owned()));
  assert_eq!(
    map.get("some_longer_name"),
    Some(
      &"hello I can do long texts too, with whitespaces!".to_owned()
    )
  );
  assert_eq!(map.get("export"), Some(&"WORKS".to_owned()));
  assert_eq!(map.get("line_break"), Some(&"\\".to_owned()));
  assert_eq!(
    map.get("i_will_be_appended"),
    Some(&"I will be appended".to_owned())
  );
  assert_eq!(map.get("num"), Some(&"0".to_owned()));
}
