// Second sugggested exercise from The Rust Programming Language Chapter 3 Summary
// Calculate fibonnaci numbers
// https://en.wikipedia.org/wiki/Fibonacci_number

use std::io;

fn main() {
    println!("Get fibonacci numbers, enter n:");

    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read...");

        let n = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let fib = fibonacci(n);

        println!("Result: {}", fib);
    }
}

fn fibonacci(n: u32) -> u32 {
    // Recursion with pattern matching
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
