use std::collections::HashMap;
use std::num::ParseFloatError;

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

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        if let Some(&callback) = self.flags.get(&flag) {
            if argv.len() >= 2 {
                match callback(argv[0], argv[1]) {
                    Ok(result) => result,
                    Err(e) => e.to_string(),
                }
            } else {
                "Invalid number of arguments".to_string()
            }
        } else {
            "Error: Flag not found".to_string()
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = a.parse::<f32>()?;
    let b = b.parse::<f32>()?;
    Ok((a / b).to_string())
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = a.parse::<f32>()?;
    let b = b.parse::<f32>()?;
    Ok((a % b).to_string())
}