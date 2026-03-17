/// Functions for palindrome calculations
/// Used primarily for Project Euler Problem 4

/// Check if a number is a palindrome by reversing its digits
/// Returns true if the number reads the same forwards and backwards
pub fn is_palindrome(n: i64) -> bool {
    let mut original = n;
    let mut reversed = 0;
    while original > 0 {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }
    return n == reversed;
}

/// Find the largest palindrome that is the product of two n-digit numbers
/// Searches through all combinations of n-digit numbers to find the largest palindromic product
pub fn find_largest_palindrome(n: i64) -> i64 {
    let mut max_palindrome = 0;
    let start = 10_i64.pow((n - 1) as u32);
    let end = 10_i64.pow(n as u32) - 1;
    for i in (start..=end).rev() {
        for j in (start..=end).rev() {
            let product = i * j;
            if is_palindrome(product) && product > max_palindrome {
                max_palindrome = product;
            }
        }
    }
    return max_palindrome;
}

pub fn find_largest_palindrome_reversed(n: i64) -> i64 {
    let mut max_palindrome = 0;
    let start = 10_i64.pow((n - 1) as u32);
    let end = 10_i64.pow(n as u32) - 1;
    for i in (start..=end).rev() {
        if i * end < max_palindrome {
            break;
        }
        for j in (i..=end).rev() {
            let product = i * j;
            if product < max_palindrome {
                break;
            }
            if is_palindrome(product) && product > max_palindrome {
                max_palindrome = product;
            }
        }
    }
    return max_palindrome;
}
