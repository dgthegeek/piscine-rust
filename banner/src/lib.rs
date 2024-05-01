use std::collections::HashMap;
use std::num::ParseFloatError;
use std::fmt;

// Definition of a general error type
#[derive(Debug)]
pub enum MyError {
    ParseError(ParseFloatError),
    ZeroDivisionError,
    Other(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MyError::ParseError(ref _e) => write!(f, "invalid float literal"),
            MyError::ZeroDivisionError => write!(f, "division by zero"),
            MyError::Other(ref msg) => write!(f, "{}", msg),
        }
    }
}


impl From<ParseFloatError> for MyError {
    fn from(err: ParseFloatError) -> MyError {
        MyError::ParseError(err)
    }
}

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: format!("-{}", l_h.chars().next().unwrap()),
            long_hand: format!("--{}", l_h),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, MyError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&self, flag: (String, String), argv: &[&str]) -> String {
        if argv.len() < 2 {
            return "not enough arguments".to_string();
        }
        match self.flags.get(&flag) {
            Some(&func) => match func(argv[0], argv[1]) {
                Ok(result) => result,
                Err(e) => e.to_string(),
            },
            None => "flag not found".to_string(),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, MyError> {
    let x = match a.parse::<f32>() {
        Ok(num) => num,
        Err(e) => return Err(MyError::ParseError(e)),
    };

    let y = match b.parse::<f32>() {
        Ok(num) => num,
        Err(e) => return Err(MyError::ParseError(e)),
    };

    Ok((x / y).to_string())  // This will automatically yield "inf" or "-inf" if y is 0.0


}

pub fn rem(a: &str, b: &str) -> Result<String, MyError> {
    let x = match a.parse::<f32>() {
        Ok(num) => num,
        Err(e) => return Err(MyError::ParseError(e)),
    };

    let y = match b.parse::<f32>() {
        Ok(num) => num,
        Err(e) => return Err(MyError::ParseError(e)),
    };

    Ok((x % y).to_string())  // This will automatically yield "inf" or "-inf" if y is 0.0

}

