fn main() {
    println!("Integer Statistics");

    let mut nums = vec![1, 3, 2];
    // sort_unstable preferred over sort
    // when stability is not needed as
    // it is faster in most cases
    // See https://doc.rust-lang.org/std/primitive.slice.html#method.sort
    nums.sort_unstable();

    println!("I have {} numbers", nums.len());

    for num in nums {
        println!("num: {}", num)
    }
}

