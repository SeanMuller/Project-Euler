use mod_exp::mod_exp;
use rand::Rng;
use std::cmp::max;
/// Functions for prime number calculations and factorization
/// Used primarily for Project Euler Problem 3
use std::collections::HashMap;

/// Find all prime factors of a given number
/// Returns a vector of prime factors in ascending order
pub fn prime_factors(mut n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            n /= i;
            i = 2;
        } else {
            i += 1;
        }
    }
    if n > 1 {
        factors.push(n);
    }
    return factors;
}

pub fn prime_factors_optimized(mut n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut i = 2;
    let mut max_factor = n.isqrt();
    while i <= max_factor {
        if n % i == 0 {
            factors.push(i);
            n /= i;
            max_factor = n.isqrt();
        } else {
            if i == 2 {
                i = 3;
            } else {
                i += 2;
            }
        }
    }
    if n > 1 {
        factors.push(n);
    }
    return factors;
}

/// Find all prime factors of a given number and group them by their value
/// Returns a hashmap where each key is a prime factor and its value is the count
pub fn prime_factors_grouped(mut n: i64) -> HashMap<i64, i64> {
    let mut factors = HashMap::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let count = factors.entry(i).or_insert(0);
            *count += 1;
            n /= i;
        } else {
            i += 1;
        }
    }
    if n > 1 {
        let count = factors.entry(n).or_insert(0);
        *count += 1;
    }
    return factors;
}

pub fn get_common_factors(vec: Vec<i64>) -> HashMap<i64, i64> {
    let mut result_factors = HashMap::new();
    for n in vec {
        let num_factors = prime_factors_grouped(n);
        for (key, value) in num_factors {
            let current_count = result_factors.entry(key).or_insert(0);
            *current_count = max(*current_count, value);
        }
    }
    return result_factors;
}

/// Find the largest prime factor of a given number
/// Uses prime_factors function and returns the last (largest) factor
pub fn largest_prime_factor(n: i64) -> i64 {
    let factors = prime_factors(n);
    return *factors.last().unwrap();
}

pub fn largest_prime_factor_optimized(n: i64) -> i64 {
    let factors = prime_factors_optimized(n);
    return *factors.last().unwrap();
}

/// Find the smallest multiple of a list of numbers up to N
pub fn smallest_multiple(n: i64) -> i64 {
    let mut vec = Vec::new();
    for i in 1..=n {
        vec.push(i);
    }
    let factors = get_common_factors(vec);
    let mut multiple = 1;
    for (key, value) in factors {
        multiple *= key.pow(value as u32);
    }
    return multiple;
}

pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let max_check = (n as f64).sqrt().ceil() as i64 + 1;
    for i in 2..max_check {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

#[allow(dead_code)]
fn miller_rabin(n: i64) -> bool {
    let s = 5;
    let k = 10;
    for _ in 0..k {
        let a = rand::rng().random_range(2..n - 1);
        let mut x = mod_exp(a, n - 1, n);
        for _ in 0..s {
            let y = mod_exp(x, 2, n);
            if y == 1 && x != 1 && x != n - 1 {
                return false;
            }
            x = y;
        }
        if x != 1 {
            return false;
        }
    }
    return true;
}

pub fn nth_prime(n: i64) -> i64 {
    let mut count = 0;
    let mut i = 2;
    while count < n {
        if is_prime(i) {
            count += 1;
        }
        i += 1;
    }
    return i - 1;
}

pub fn prime_sum(n: i64) -> i64 {
    let mut sum = 0;
    for i in 2..n {
        if is_prime(i) {
            sum += i;
        }
    }
    return sum;
}
