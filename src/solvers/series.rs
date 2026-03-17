use std::fs;

fn largest_product_of_consecutive_digits(digits: Vec<i64>, n: i64) -> i64 {
    let mut max_product = 0;
    for i in 0..digits.len() - n as usize {
        let product = digits[i..i + n as usize].iter().product();
        if product > max_product {
            max_product = product;
        }
    }
    return max_product;
}

pub fn largest_product_in_series(n: i64) -> i64 {
    // Read the problem 8 file
    let digits = fs::read_to_string("./data/problem_8.txt").expect("Failed to read file");
    let digits: Vec<i64> = digits.chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
    return largest_product_of_consecutive_digits(digits, n);
}