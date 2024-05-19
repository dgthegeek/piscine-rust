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
impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        use RomanDigit::*;
        let mut digits = Vec::new();
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }
        while num > 0 {
            digits.extend(match num {
                _ if num >= 1000 => { num -= 1000; vec![M] }
                _ if num >= 900 => { num -= 900; vec![C, M] }
                _ if num >= 500 => { num -= 500; vec![D] }
                _ if num >= 400 => { num -= 400; vec![C, D] }
                _ if num >= 100 => { num -= 100; vec![C] }
                _ if num >= 90 => { num -= 90; vec![X, C] }
                _ if num >= 50 => { num -= 50; vec![L] }
                _ if num >= 40 => { num -= 40; vec![X, L] }
                _ if num >= 10 => { num -= 10; vec![X] }
                _ if num >= 9 => { num -= 9; vec![I, X] }
                _ if num >= 5 => { num -= 5; vec![V] }
                _ if num >= 4 => { num -= 4; vec![I, V] }
                _ => { num -= 1; vec![I] }
            });
        }
        RomanNumber(digits)
    }
}