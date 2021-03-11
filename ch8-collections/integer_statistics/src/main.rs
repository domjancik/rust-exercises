fn main() {
    println!("Integer Statistics");

    let mut nums = vec![1, 3, 2, 8, 10];
    // sort_unstable preferred over sort
    // when stability is not needed as
    // it is faster in most cases
    // See https://doc.rust-lang.org/std/primitive.slice.html#method.sort
    nums.sort_unstable();

    let len = nums.len();
    println!("I have {} numbers", nums.len());
    println!("{:?}", nums);
    println!("");

    let median_index = len / 2;
    // We are fine both with and with the & reference operator
    // Though likely only because
    // we are dealing with a primitive type with Copy
    let median = &nums[median_index];
    println!("Median Index: {}", median_index);
    println!("Median is: {}", median);

    for num in &nums {
        println!("num: {}", num)
    }
}

