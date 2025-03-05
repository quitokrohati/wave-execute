use std::fs;

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub fn write_file(file_path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(file_path, content)
}