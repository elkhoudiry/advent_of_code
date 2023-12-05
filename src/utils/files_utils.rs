use std::env;
use std::fs;

pub fn get_file_contents(relative_path: &str) -> String {
    let path = env::current_dir().unwrap().to_str().unwrap().to_string();
    let path_str = format!("{}{}", path, relative_path);
    let contents = fs::read_to_string(path_str).expect("Should have been able to read the file");

    contents
}
