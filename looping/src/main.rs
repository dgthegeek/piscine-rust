use std::io;

fn main() {
    let answer = String::from("The letter e");
    let mut counter = 0;

    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

        let mut guess = String::new();
        counter += 1;

        io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");

        if guess.trim() == answer {
            println!("Number of trials: {}", counter);
            break
        }
    }
}
