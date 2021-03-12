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

            // TODO check for vowel
            println!("{}-{}ay", rest, first_letter);
        }
    }
}
