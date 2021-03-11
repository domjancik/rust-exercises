use std::io;

fn main() {
    println!("Feed me words, oink!");

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Read ailed-fay :(");
        let words = line.split_whitespace();

        for word in words {
            let first_letter = word.chars()[0]
            println!("{}", first_letter);
        }
    }
}
