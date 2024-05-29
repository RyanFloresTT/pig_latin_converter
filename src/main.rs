use std::io;

fn main() {
    println!("Enter a phrase to be converted to Pig Latin : ");
    let mut user_phrase = String::new();
    io::stdin()
        .read_line(&mut user_phrase)
        .expect("Failed to read line");    
    let trimmed_phrase = user_phrase.trim();

    for word in trimmed_phrase.split_whitespace() {
        let mut word_chars = word.chars();
        let first_letter = word_chars.next().unwrap().to_lowercase();
        let first_rest = word_chars.next().unwrap().to_uppercase();
        let mut rest: String = word_chars.collect();
        let mut punctuation = String::new();
        if let Some(last_char) = rest.chars().last() {
            if !last_char.is_alphanumeric() {
                punctuation = rest.pop().unwrap().to_string();
            }
        }
        print!("{first_rest}{rest}{first_letter}ay{punctuation} ");
    }
}
