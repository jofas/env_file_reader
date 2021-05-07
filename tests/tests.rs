use env_file_reader::read_file;

#[test]
fn env_file1() {
  let map = read_file("examples/env_file1").unwrap();
  assert_eq!(map.get("a"), Some(&"C".to_owned()));
  assert_eq!(map.get("b"), Some(&"C".to_owned()));
  assert_eq!(map.get("some_longer_name"), Some(&"hello I can do long texts too, with whitespaces!".to_owned()));
  assert_eq!(map.get("export"), Some(&"WORKS".to_owned()));
  assert_eq!(map.get("line_break"), Some(&"\\".to_owned()));
}
