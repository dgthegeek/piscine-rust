use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = String::from("The letter e");
    let mut tries = 0;

    loop {
        println!("{}", riddle);
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        if user_input.trim() == answer {
            tries += 1;
            println!("Number of trials: {}", tries);
            break
        } else {
            tries += 1;
        }
    }
}

