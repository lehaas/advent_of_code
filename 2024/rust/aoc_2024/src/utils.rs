use std::fs;
use std::path::Path;

/// Read the content of a file into a String.
pub fn read_file_content(file_path: impl AsRef<Path>) -> String {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    content
}
