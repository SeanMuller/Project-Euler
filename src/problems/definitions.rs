use crate::solvers::palindromes::find_largest_palindrome_reversed;

use super::registry::ProblemRegistry;
use super::types::Problem;

/// Initialize and return the problem registry with all problems
pub fn get_problem_registry() -> ProblemRegistry {
    ProblemRegistry::new()
        .register(create_problem_0())
        .register(create_problem_1())
        .register(create_problem_2())
        .register(create_problem_3())
        .register(create_problem_4())
        .register(create_problem_5())
        .register(create_problem_6())
        .register(create_problem_7())
        .register(create_problem_8())
        .register(create_problem_9())
        .register(create_problem_10())
        .register(create_problem_11())
        .register(create_problem_12())
        .register(create_problem_14())
}

fn create_problem_0() -> Problem {
    use crate::solvers::squares::sum_of_odd_squares;

    Problem::new(
        0,
        "Sum of the first n odd squares",
        "Sum of the first n odd squares",
        10,
        100,
    )
    .add_solution("formula", sum_of_odd_squares)
}

/// Problem 1: Multiples of 3 and 5
fn create_problem_1() -> Problem {
    use crate::solvers::multiples::sum_of_multiples_of_3_and_5;

    Problem::new(
        1,
        "Multiples of 3 and 5",
        "Sum of multiples of 3 or 5 below given number",
        10,
        23,
    )
    .add_solution("formula", sum_of_multiples_of_3_and_5)
}

/// Problem 2: Even Fibonacci Numbers
fn create_problem_2() -> Problem {
    use crate::solvers::fibonacci::{
        sum_of_even_fibonacci_numbers, sum_of_even_fibonacci_numbers_simple,
    };

    Problem::new(
        2,
        "Even Fibonacci Numbers",
        "Sum of even Fibonacci numbers below given number",
        90,
        44, // 2 + 8 + 34 = 44 (even Fibonacci numbers below 90)
    )
    .add_solution("reduced", sum_of_even_fibonacci_numbers)
    .add_solution("simple", sum_of_even_fibonacci_numbers_simple)
}

/// Problem 3: Largest Prime Factor
fn create_problem_3() -> Problem {
    use crate::solvers::primes::{largest_prime_factor, largest_prime_factor_optimized};

    Problem::new(
        3,
        "Largest Prime Factor",
        "Largest prime factor of given number",
        13195,
        29,
    )
    .add_solution("factorization", largest_prime_factor)
    .add_solution("factorization_optimized", largest_prime_factor_optimized)
}

/// Problem 4: Largest Palindrome Product
fn create_problem_4() -> Problem {
    use crate::solvers::palindromes::find_largest_palindrome;

    Problem::new(
        4,
        "Largest Palindrome Product",
        "Largest palindrome from product of two n-digit numbers",
        2,
        9009, // 91 × 99 = 9009
    )
    .add_solution("brute_force", find_largest_palindrome)
    .add_solution("brute_force_reversed", find_largest_palindrome_reversed)
}

/// Problem 5: Smallest Multiple
fn create_problem_5() -> Problem {
    use crate::solvers::primes::smallest_multiple;

    Problem::new(
        5,
        "Smallest Multiple",
        "Smallest multiple of all numbers up to given number",
        10,
        2520,
    )
    .add_solution("lcm", smallest_multiple)
}

/// Problem 6: Sum Square Difference
fn create_problem_6() -> Problem {
    use crate::solvers::squares::difference;

    Problem::new(
        6,
        "Sum Square Difference",
        "Difference between square of sum and sum of squares",
        10,
        2640, // (1+2+...+10)² - (1²+2²+...+10²) = 55² - 385 = 3025 - 385 = 2640
    )
    .add_solution("formula", difference)
}

/// Problem 7: 10001st Prime
fn create_problem_7() -> Problem {
    use crate::solvers::primes::nth_prime;

    Problem::new(
        7,
        "10001st Prime",
        "Find the nth prime number",
        6,
        13, // 6th prime is 13
    )
    .add_solution("sieve", nth_prime)
}

/// Problem 8: Largest Product in a Series
fn create_problem_8() -> Problem {
    use crate::solvers::series::largest_product_in_series;

    Problem::new(
        8,
        "Largest Product in a Series",
        "Largest product of n adjacent digits in a given number",
        4,
        5832,
    )
    .add_solution("brute_force", largest_product_in_series)
}

/// Problem 9: Special Pythagorean Triplet
fn create_problem_9() -> Problem {
    use crate::solvers::squares::pythagorean_triplet_product_equal_to;

    Problem::new(
        9,
        "Special Pythagorean Triplet",
        "Product of Pythagorean triplet that sums to given number",
        12,
        60, // 3² + 4² = 5², 3+4+5=12, 3*4*5=60
    )
    .add_solution("search", pythagorean_triplet_product_equal_to)
}

/// Problem 10: Summation of Primes
fn create_problem_10() -> Problem {
    use crate::solvers::primes::prime_sum;

    Problem::new(
        10,
        "Summation of Primes",
        "Sum of all primes below given number",
        10,
        17, // 2 + 3 + 5 + 7 = 17
    )
    .add_solution("sieve", prime_sum)
}

fn create_problem_11() -> Problem {
    use crate::solvers::grid::largest_product_in_grid_11;

    Problem::new(
        11,
        "Largest Product in a Grid",
        "Largest product of n adjacent digits in a given grid",
        4,
        5832,
    )
    .add_solution("brute_force", largest_product_in_grid_11)
}

fn create_problem_12() -> Problem {
    use crate::solvers::triangle_numbers::highly_divisible_triangular_number;

    Problem::new( 
        12, 
        "Highly Divisible Triangular Number", 
        "The first triangle number to have over N divisors", 
        5, 
        28,
    )
    .add_solution("brute_force", highly_divisible_triangular_number)
}


fn create_problem_14() -> Problem{
    use crate::solvers::collatz::longest_collatz;

    Problem::new(14, "Longest Collatz Sequence", "Find the starting number less than N to produce the longest collatz sequence", 20, 18)
    .add_solution("brute_force", longest_collatz)
}