// Third sugggested exercise from The Rust Programming Language Chapter 3 Summary
// Generate lyrics for 12 Days of Christmas
// https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics

// consts must be typed explicitly
const FIRST_LINE_PRE: &str = "On the";
const FIRST_LINE_POST: &str = "day of Christmas, my true love sent to me";
const ORDINALS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];

const VERSE_LINES: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    for verse in 0..12 {
        println!("{} {} {}", FIRST_LINE_PRE, ORDINALS[verse], FIRST_LINE_POST);

        for line in VERSE_LINES[0..verse + 1].iter().rev() {
            println!("{}", line);
        }

        println!("");
    }
}
