use env_file_reader::{read_file, read_files};

#[test]
fn test_read_file() {
  let map = read_file("examples/env_file1").unwrap();

  assert_eq!(map.get("A"), Some(&"C".to_owned()));
  assert_eq!(map.get("B"), Some(&"C".to_owned()));

  assert_eq!(
    map.get("SOME_LONGER_NAME"),
    Some(
      &"hello I can do long texts too, with whitespaces!".to_owned()
    )
  );

  assert_eq!(map.get("EXPORT"), Some(&"WORKS".to_owned()));

  assert_eq!(map.get("LINE_BREAK"), Some(&"\\".to_owned()));

  assert_eq!(map.get("FOO"), Some(&"bar".to_owned()));
  assert_eq!(map.get("Foo"), Some(&"baz".to_owned()));
  assert_eq!(map.get("foo"), Some(&"qux".to_owned()));

  assert_eq!(
    map.get("this-variable-name"),
    Some(&"is-ok".to_owned()),
  );
  assert_eq!(map.get("123_this-too"), Some(&"true".to_owned()));
}

#[test]
fn test_read_files() {
  let files = vec!["examples/env_file1", "examples/env_file2"];
  let map = read_files(&files).unwrap();

  assert_eq!(
    map.get("I_WILL_BE_APPENDED"),
    Some(&"I will be appended".to_owned())
  );

  assert_eq!(map.get("NUM"), Some(&"0".to_owned()));

  assert_eq!(map.get("A"), Some(&"D".to_owned()));
}
