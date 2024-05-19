use std::fs::File;

pub fn open_file(s: &str) -> File {
    let file_result = File::open(s);
    let file = match file_result {
        Ok(file) => file,
        Err(_) => panic!("File not found"),
    };
    file
}
