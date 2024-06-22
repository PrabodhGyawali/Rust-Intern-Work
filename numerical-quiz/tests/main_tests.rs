use std::process::Command;
use std::process::Output;
use std::str;

#[test]
fn test_args_path_no_message() {
    let output = Command::new("cargo")
        .args(&["run", "--", "arg1", "--no-message"])
        .output()
        .expect("");
    let ouput_string = str::from_utf8(&output.stdout).unwrap();
    println!("{:?}", ouput_string);
    assert!(!ouput_string.contains("This is a numerical quiz game on songs.\n"));
}

#[test]
fn test_invalid_arg() {
    let output = Command::new("cargo")
        .args(&["run", "--", "arg1", "--invalid"])
        .output()
        .expect("");
    let err_string = str::from_utf8(&output.stderr).unwrap();
    println!("{:?}", err_string);
    assert!(err_string.contains("Invalid option\n"));
}

#[test]
fn test_args_h() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-h"])
        .output()
        .expect(
            "Help message: This program plays sounds from a file.\n\
                  cargo run [path] [options] -> the path to the file relative to cargo.toml\n\
                  --no-message -> optional parameter to not display message",
        );
    let output_str = str::from_utf8(&output.stdout).unwrap();
    let expected_output = "Help message: This program plays sounds from a file.\n\
                  cargo run [path] [options] -> the path to the file relative to cargo.toml\n\
                  --no-message -> optional parameter to not display message";
    println!("{:?}", expected_output);
    assert_eq!(output_str.trim(), expected_output);
}
