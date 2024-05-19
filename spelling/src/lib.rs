pub fn spell(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }

    let mut result = String::new();
    let ones = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n >= 1_000_000 {
        result.push_str(&spell(n / 1_000_000));
        result.push_str(" million");
        if n % 1_000_000 != 0 {
            result.push(' ');
            result.push_str(&spell(n % 1_000_000));
        }
    } else if n >= 1_000 {
        result.push_str(&spell(n / 1_000));
        result.push_str(" thousand");
        if n % 1_000 != 0 {
            result.push(' ');
            result.push_str(&spell(n % 1_000));
        }
    } else if n >= 100 {
        result.push_str(ones[(n / 100) as usize]);
        result.push_str(" hundred");
        if n % 100 != 0 {
            result.push(' ');
            result.push_str(&spell(n % 100));
        }
    } else if n >= 20 {
        result.push_str(tens[(n / 10) as usize]);
        if n % 10 != 0 {
            result.push('-');
            result.push_str(ones[(n % 10) as usize]);
        }
    } else {
        if n == 10 {
            result.push_str("ten");
        } else if n == 0 {
            result.push_str("zero");
        } else if n <= 9 {
            result.push_str(ones[n as usize]);
        } else {
            result.push_str(teens[(n - 10) as usize]);
        }
    }
    
    result
}