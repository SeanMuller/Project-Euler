use super::types::BenchmarkConfig;
use std::collections::HashMap;

/// Default benchmark configurations for each problem
pub fn get_default_benchmark_configs() -> HashMap<i64, BenchmarkConfig> {
    let mut configs = HashMap::new();

    // Problem 1: Multiples of 3 and 5
    configs.insert(1, BenchmarkConfig::new(1, 10, 1000000, 5.0));

    // Problem 2: Even Fibonacci Numbers
    configs.insert(2, BenchmarkConfig::new(2, 90, 40000000, 5.0));

    // Problem 3: Largest Prime Factor
    configs.insert(3, BenchmarkConfig::new(3, 13195, 600851475143, 10.0));

    // Problem 4: Largest Palindrome Product
    configs.insert(4, BenchmarkConfig::new(4, 2, 10, 5.0)); // 2-digit to 4-digit

    // Problem 5: Smallest Multiple
    configs.insert(5, BenchmarkConfig::new(5, 10, 30, 10.0));

    // Problem 6: Sum Square Difference
    configs.insert(6, BenchmarkConfig::new(6, 10, 10000, 5.0));

    // Problem 7: Nth Prime
    configs.insert(7, BenchmarkConfig::new(7, 6, 10001, 15.0));

    // Problem 8: Largest Product in Series
    configs.insert(8, BenchmarkConfig::new(8, 4, 13, 5.0));

    // Problem 9: Pythagorean Triplet
    configs.insert(9, BenchmarkConfig::new(9, 12, 1000, 10.0));

    // Problem 10: Sum of Primes
    configs.insert(10, BenchmarkConfig::new(10, 10, 2000000, 20.0));

    configs
}
