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
            match first_letter {
                // | allows matching multiple cases at once. Also ranges usinng .. are supported.
                // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
                'a' | 'e' | 'i' | 'o' | 'u' => transform_vowel_word(&first_letter, &rest),
                _ => transform_consonant_word(&first_letter, &rest)
            }
        }
    }
}

fn transform_vowel_word(first_letter: &char, rest: &String) {
    println!("{}{}-hay", first_letter, rest);
}

fn transform_consonant_word(first_letter: &char, rest: &String) {
    println!("{}-{}ay", rest, first_letter);
}