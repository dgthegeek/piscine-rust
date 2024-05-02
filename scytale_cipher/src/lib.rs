pub fn scytale_cipher(message: String, wraps: u32) -> String {
    if wraps >= message.len() as u32 || wraps == 0 {
        return message;
    }

    let wraps = wraps as usize;
    let message_len = message.len();
    let mut result = String::with_capacity(message_len);

    let rows = (message_len + wraps - 1) / wraps;

    for col in 0..wraps {
        for row in 0..rows {
            let index = row * wraps + col;
            if index < message_len {
                result.push(message.chars().nth(index).unwrap());
            } else {
                result.push(' ');
            }
        }
    }

    result.trim_end().to_string()
}
