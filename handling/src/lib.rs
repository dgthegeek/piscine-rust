use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(file)
                .expect("Failed to open or create the file");
    if let Err(err) = file.write_all(content.as_bytes()) {
        panic!("Failed to write to the file: {}", err);
    }
}
