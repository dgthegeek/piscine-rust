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
    fn from(n: u32) -> Self {
        match n {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!(),
        }
    }
}
impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self{
        if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }
        
        let mut result = Vec::new();
        let div = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        
        for (i, n) in div.iter().enumerate() {
            while n <= &num {
                if i % 2 == 0 {
                    result.push(RomanDigit::from(*n));
                }else{
                    let rem = div[i - 1] - div[i];
                    result.push(RomanDigit::from(rem));
                    result.push(RomanDigit::from(div[i - 1]));
                }
                num -= n;
            }
        }
        RomanNumber(result)
    }
}
impl Iterator for RomanNumber {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        // Convert the current RomanNumber to an integer
        let mut num = 0;
        let mut last_digit_value = 0;
        for digit in &self.0 {
            let value = match digit {
                RomanDigit::Nulla => 0,
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1000,
            };
            if value > last_digit_value {
                num += value - 2 * last_digit_value;
            } else {
                num += value;
            }
            last_digit_value = value;
        }
        // Increment the integer
        num += 1;
        // Convert the incremented integer back to a RomanNumber
        *self = RomanNumber::from(num);
        Some(self.clone())
    }
}
