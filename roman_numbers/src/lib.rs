use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
	fn from(value: u32) -> Self {
		match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => todo!(),
        }
	}
}

impl From<u32> for RomanNumber {
	fn from(value: u32) -> Self {
		let mut num = value;
		let mut roman_digits = Vec::new();
        let  powers_of_ten = vec![1000, 100, 10, 1];
        let mut index = 0;

		if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        while num > 0 {
            let digit = num / powers_of_ten[index];
            if digit > 0 {
                match digit {
                    1..=3 => {
                        for _ in 0..digit {
                            roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        }
                    }
                    4 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        roman_digits.push(RomanDigit::from(powers_of_ten[index] * 5));
                    }
                    5 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index] * 5));
                    }
                    6..=8 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index] * 5));
                        for _ in 0..(digit - 5) {
                            roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        }
                    }
                    9 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        roman_digits.push(RomanDigit::from(powers_of_ten[index - 1]));
                    }
                    _ => unreachable!(),
                }
            }
            num %= powers_of_ten[index];
            index += 1;
        }
        
        RomanNumber(roman_digits)
	}
	
}

// pub fn roman_number(value: u32) -> String {
// 	let mut num = value;
//     let int_array = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
//     let roman_array = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
//     let mut roman_number = String::new();

//     for i in 0..int_array.len() {
//         while num >= int_array[i] {
//             num -= int_array[i];
//             roman_number.push_str(roman_array[i]);
//         }
//     }

//     roman_number
// }