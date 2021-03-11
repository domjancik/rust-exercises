use std::collections::HashMap;

fn main() {
    println!("Integer Statistics");

    let mut nums = vec![1, 3, 2, 8, 10, 10, 8, 8];
    // sort_unstable preferred over sort
    // when stability is not needed as
    // it is faster in most cases
    // See https://doc.rust-lang.org/std/primitive.slice.html#method.sort
    nums.sort_unstable();

    let len = nums.len();
    println!("I have {} numbers, ordered them for you:", nums.len());
    println!("{:?}", nums);
    println!("");

    println!("Let's take a closer look...");
    let median_index = len / 2;
    // We are fine both with and with the & reference operator
    // Though likely only because
    // we are dealing with a primitive type with Copy
    let median = &nums[median_index];
    println!("Median is: {} (at index {})", median, median_index);

    // .sum() is part of the iter(), not vec itself!
    let sum: u32 = nums.iter().sum();
    println!("Sum is: {}", sum);

    // More on casting and conversions
    // https://doc.rust-lang.org/rust-by-example/types/cast.html
    let average = sum as f32 / len as f32;
    println!("Average is: {}", average);

    let mut mode_map = HashMap::new();
    for num in &nums {
        let count = mode_map.entry(num).or_insert(0);
        *count += 1;
        // println!("   Counting {}, found {}", num, count);
    }

    // First time I see rust's lambdas, nice syntax
    // https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.max_by_key
    let mode = mode_map.iter().max_by_key(|x| x.1);
    match mode {
        Some(m) => println!("Mode: {} (Count {})", m.0, m.1),
        None => println!("Mode: No numbers to count 'm afraid...")
    }
}

