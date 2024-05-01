use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug)]
pub struct FlagError {
    message: String,
}

impl fmt::Display for FlagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for FlagError {}

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(s_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: format!("-{}", s_h),
            long_hand: format!("--{}", s_h),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, Box<dyn Error>>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> Result<String, Box<dyn Error>> {
        if let Some(func) = self.flags.get(&flag) {
            let (a, b) = (argv.get(0).unwrap_or(&""), argv.get(1).unwrap_or(&""));
            func(*a, *b)
        } else {
            Err(Box::new(FlagError { message: "Flag not found".to_string() }))
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, Box<dyn Error>> {
    let a: f32 = a.parse().map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let b: f32 = b.parse().map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, Box<dyn Error>> {
    let a: f32 = a.parse().map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let b: f32 = b.parse().map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok((a % b).to_string())
}
