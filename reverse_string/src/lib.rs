pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}

// The method we used above convert the string into a collection of characters chars(),
// reverse the collection rev(), and convert the collection back into a string collect()


// We could also use the following function:
// pub fn rev_str(input: &str) -> String {
//     let mut reversed = String::new();
//     for char in input.chars().rev() {
//         reversed.push(char);
//     }
//     reversed
// }
