#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
	type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" => Ok(Antigen::A),
			"B" => Ok(Antigen::B),
			"AB" => Ok(Antigen::AB),
			"O" => Ok(Antigen::O),
			_ => Err(())
		}
	}
	
}

impl FromStr for RhFactor {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(RhFactor::Positive),
			"-" => Ok(RhFactor::Negative),
			_ => Err(())
		}
	}
}

impl Ord for BloodType {
	
	fn cmp(&self, other: &Self) -> Ordering {
		if self.antigen != other.antigen{
			self.antigen.cmp(&other.antigen)
		}else{
			self.rh_factor.cmp(&other.rh_factor)
		}
	}
}

impl FromStr for BloodType {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let antigen = Antigen::from_str(&s[..s.len()-1])?;
		let rh_factor = RhFactor::from_str(&s[s.len()-1..])?;
		Ok(BloodType{ antigen, rh_factor})
	}
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f,
			"{}{}",
			match self.antigen {
				Antigen::A => "A",
				Antigen::B => "B",
				Antigen::AB => "AB",
				Antigen::O => "O"
			},
			match self.rh_factor {
				RhFactor::Positive => "+",
				RhFactor::Negative => "-"
			}
		)
	}
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
		match (self.antigen.clone(), self.rh_factor.clone()) {
			(Antigen::A, RhFactor::Positive) => match (other.antigen.clone(), other.rh_factor.clone()){
				(Antigen::A | Antigen::O, RhFactor::Positive | RhFactor::Negative) => true,
				_ => false
			}
			(Antigen::A, RhFactor::Negative) => match (other.antigen.clone(), other.rh_factor.clone()){
				(Antigen::A | Antigen::O, RhFactor::Negative) => true,
				_ => false
			}
			(Antigen::B, RhFactor::Positive) => match (other.antigen.clone(), other.rh_factor.clone()){
				(Antigen::B | Antigen::O, RhFactor::Positive | RhFactor::Negative) => true,
				_ => false
			}
			(Antigen::B, RhFactor::Negative) => match (other.antigen.clone(), other.rh_factor.clone()){
				(Antigen::B | Antigen::O, RhFactor::Negative) => true,
				_ => false
			}
			(Antigen::AB, RhFactor::Positive) => match (other.antigen.clone(), other.rh_factor.clone()) {
				(Antigen::AB | Antigen::A | Antigen::B | Antigen::O, RhFactor::Positive | RhFactor::Negative) => true,
				_ => false
			}
			(Antigen::AB, RhFactor::Negative) => match (other.antigen.clone(), other.rh_factor.clone()) {
				(Antigen::AB | Antigen::A | Antigen::B | Antigen::O, RhFactor::Negative) => true,
				_ => false
			}
			(Antigen::O, RhFactor::Positive) => match (other.antigen.clone(), other.rh_factor.clone()){
				(Antigen::O, RhFactor::Negative | RhFactor::Positive) => true,
				_ => false
			}
			(Antigen::O, RhFactor::Negative) => match (other.antigen.clone(), other.rh_factor.clone()){
				(Antigen::O, RhFactor::Negative) => true,
				_ => false
			}
		}
	}

	pub fn donors(&self) -> Vec<Self> {
        let all_blood_types = [
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
        ];

        let mut donors = Vec::new();

        for blood_type in all_blood_types {
            if self.can_receive_from(&blood_type) {
                donors.push(blood_type);
            }
        }

        donors
    }

	pub fn recipients(&self) -> Vec<BloodType> {
		 let all_blood_types = [
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
        ];

        let mut recipients = Vec::new();

        for blood_type in all_blood_types {
            if blood_type.can_receive_from(&self) {
				let a = blood_type;
                recipients.push(a);
            }
        }

        recipients
	}
}
