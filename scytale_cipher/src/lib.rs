pub fn scytale_cipher(message: String, size: u32) -> String {
    let mut result = String::new();
    let mut matrix: Vec<Vec<Option<char>>> = vec![vec![None; size as usize]; (message.len() as f32 / size as f32).ceil() as usize];
    
    for (i, c) in message.chars().enumerate() {
        let row = i / size as usize;
        let col = i % size as usize;
        matrix[row][col] = Some(c);
    }

    for col in 0..size as usize {
        for row in 0..matrix.len() {
            if let Some(c) = matrix[row][col] {
                result.push(c);
            }else {
                result.push(' ')
            }
        }
    }
    
    result.trim().to_string()
}