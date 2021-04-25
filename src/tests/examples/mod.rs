use std::path::Path;
use std::fs::File;
use std::io::*;
use std::env;

use io_helper;


/// Get the example file by the filename
fn get_example_file(filename: &str) -> Result<File> {
    let root = env::current_dir().unwrap();
    let path = Path::join(&root, format!("src/tests/examples/{}", filename));
    File::open(path)
}


/// Create a new Cursor
fn new_cursor() -> Cursor<Vec<u8>> {
    Cursor::new(vec! { })
}


/// Convert a cursor to a string
fn cursor_to_string(cursor: Cursor<Vec<u8>>) -> String {
    cursor
        .into_inner()
        .into_iter()
        .map(|x| x as char) // u8 is not collected to String, so convert to char
        .collect()
}


/// Execute a file, then return the output
fn get_exec_output_file(filename: &str, input: Option<&str>) -> String {
    let file = get_example_file(filename).unwrap();
    let mut in_cursor = new_cursor();
    let mut out_cursor = new_cursor();

    if let Some(input) = input {
        in_cursor.write_all(input.as_bytes()).expect("Failed to write input");
    }

    io_helper::execute_file(file, &mut out_cursor);

    cursor_to_string(out_cursor)
}


// TODO: Fix these tests
// NOTE: Output should be run through output language first

#[test]
#[ignore]
fn hello_world() {
    let output = get_exec_output_file("hello_world.glass", None);

    assert_eq!(output, "Hello, World!");
}


#[test]
#[ignore]
fn cat() {
    let input = "Hello, World!";
    let output = get_exec_output_file("cat.glass", Some(input));

    assert_eq!(output, "Hello, World!");
}


#[test]
#[ignore]
fn counter() {
    let output = get_exec_output_file("counter.glass", None);

    assert_eq!(output, "");
}


#[test]
#[ignore]
fn fibonacci() {
    let output = get_exec_output_file("fibonacci.glass", None);

    assert_eq!(output, "");
}
