use include_file_path::include_file_path;

#[test]
fn test_absolute_include_file_path() {
    const PATH: &str = include_file_path!("/home");
    assert!(PATH.ends_with("/home"));
}

#[test]
fn test_relative_include_file_path() {
    const PATH: &str = include_file_path!("test_file.txt");
    let contents = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(contents, "Hello World!");
}
