pub fn get_diamond(c: char) -> Vec<String> {
    let size = (c as u8 - b'A' + 1) as usize;
    let mut diamond = Vec::new();
    let mut row = 1;

    for ch in b'A'..=c as u8 {
        let spaces = size - (ch - b'A') as usize - 1;
        let mut line = String::new();
        line.push_str(&" ".repeat(spaces));
        line.push(ch as char);
        if ch != b'A' {
            line.push_str(&" ".repeat(row));
            line.push(ch as char);
            row += 2;
        }
        line.push_str(&" ".repeat(spaces));
        diamond.push(line);
    }

    for ch in (b'A'..c as u8).rev() {
        let spaces = size - (ch - b'A') as usize - 1;
        let mut line = String::new();
        line.push_str(&" ".repeat(spaces));
        line.push(ch as char);
        if ch != b'A' {
            line.push_str(&" ".repeat(row - 4));
            line.push(ch as char);
            row -= 2;
        }
        line.push_str(&" ".repeat(spaces));
        diamond.push(line);
    }
    
    diamond
}