// Rust Programming Language Chapter 8 Summary second exercise
//
// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and "ay" is added, so "first" becomes "irst-fay".
// Words that start with a vowel have "hay" added to the end instead ("apple" becomes "apple-hay").
// Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    println!("Feed me words, oink!");

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Read ailed-fay :(");
        let words = line.split_whitespace();

        for word in words {
            let mut chars = word.chars();
            let first_letter = chars.next().expect("Empty word, oops...");
            // .collect(), important!
            let rest: String = chars.collect();
            
            let word_in_pig_latin = match first_letter {
                // | allows matching multiple cases at once. Also ranges usinng .. are supported.
                // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
                'a' | 'e' | 'i' | 'o' | 'u' => transform_vowel_word(&first_letter, &rest),
                _ => transform_consonant_word(&first_letter, &rest)
            };

            print!("{} ", word_in_pig_latin);
        }

        println!("");
    }
}

fn transform_vowel_word(first_letter: &char, rest: &String) -> String {
    format!("{}{}-hay", first_letter, rest)
}

fn transform_consonant_word(first_letter: &char, rest: &String) -> String {
    format!("{}-{}ay", rest, first_letter)
}