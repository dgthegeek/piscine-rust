pub use chrono::{NaiveDate,Utc};

#[derive(Debug,Eq,PartialEq)]
pub struct FormError {
    pub form_values: (String,String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new (field_name: &str, field_value: &str, err: &str) -> FormError {
        FormError {
            form_values: (field_name.to_string(), field_value.to_string()),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err.to_string()
        }
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
            Form{
            first_name,
            last_name,
            birth,
            birth_location,
            password
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>,FormError> {
        if self.first_name.is_empty(){
            return Err(FormError::new("first_name", &self.first_name, "No user name"));
        }

        if self.password.len() < 8 {
            return Err(FormError::new("password", &self.password, "At least 8 characters"));
        }

        if !self.password.chars().any(char::is_numeric) ||
            !self.password.chars().any(char::is_alphabetic) ||
            ! self.password.chars().any(|c| !c.is_alphanumeric()) {
                return Err(FormError::new("password", &self.password, "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)"));
        }

        Ok(vec!["Valid first name", "Valid password"])

    }
}


pub fn create_date(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("Invalid date format")
}

