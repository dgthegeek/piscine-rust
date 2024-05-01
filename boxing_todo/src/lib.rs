mod err;
use err::{ParseErr, ReadErr};

pub use json::{parse, stringify};
pub use std::error::Error;
pub use std::fs;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file_content = fs::read_to_string(path).map_err(|err| ReadErr { child_err: Box::new(err) })?;
        let parsed: Result<TodoList, _> = parse(&file_content);
        parsed.map_err(|err| {
            if err.classify() == &json::ErrorCode::KeyMustBeBareString {
                ParseErr::Malformed(Box::new(err))
            } else {
                ParseErr::Empty
            }
        })
    }
}