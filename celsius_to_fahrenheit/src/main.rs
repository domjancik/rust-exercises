// First sugggested exercise from The Rust Programming Language Chapter 3 Summary
// Convert from Celsius to Fahrenheit
// http://www.srhartley.com/celsius-to-fahrenheit/formula/

use std::io;

fn main() {
    println!("Converting C° to F°, enter a number");

    loop {
        let mut celsius = String::new();
        
        io::stdin().read_line(&mut celsius).expect("Failed to read...");

        let celsius: f32 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let fahrenheit = convert_to_fahrenheit(celsius);

        println!("{} C° is {} F°", celsius, fahrenheit);
    }
}

fn convert_to_fahrenheit(celsius: f32) -> f32 {
    // line with no ending semicolon is an expression and implicitly returns
    celsius * 1.8 + 32.0
}
