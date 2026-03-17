/// Functions for Fibonacci sequence calculations
/// Used primarily for Project Euler Problem 2

/// Calculate the sum of even Fibonacci numbers below n using iterative approach
/// More efficient than recursive approach for large numbers
pub fn sum_of_even_fibonacci_numbers_simple(n: i64) -> i64 {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;
    while b < n {
        if b % 2 == 0 {
            sum += b;
        }
        let temp = a + b;
        a = b;
        b = temp;
    }
    return sum;
}

pub fn sum_of_even_fibonacci_numbers(n: i64) -> i64 {
    let mut a = 2;
    let mut b = 8;
    let mut sum = a;
    let mut c = a + b;
    while b < n {
        sum += b;
        a = b;
        b = c;
        c = a + b;
    }
    return sum;
}
