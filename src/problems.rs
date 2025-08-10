use std::collections::HashMap;

use crate::series::largest_product_in_series;

/// Type alias for solution functions
pub type SolutionFn = fn(i64) -> i64;

/// Represents a Project Euler problem with metadata and multiple solution approaches
#[derive(Clone, Debug)]
pub struct Problem {
    pub number: i64,
    pub title: &'static str,
    pub description: &'static str,
    pub example_input: i64,
    pub expected_output: i64,
    pub solutions: Vec<(&'static str, SolutionFn)>,
}

impl Problem {
    /// Create a new problem instance
    pub fn new(
        number: i64,
        title: &'static str,
        description: &'static str,
        example_input: i64,
        expected_output: i64,
    ) -> Self {
        Problem {
            number,
            title,
            description,
            example_input,
            expected_output,
            solutions: Vec::new(),
        }
    }

    /// Add a solution method to this problem
    pub fn add_solution(mut self, name: &'static str, solution_fn: SolutionFn) -> Self {
        self.solutions.push((name, solution_fn));
        self
    }

    /// Get the primary (first) solution
    pub fn get_primary_solution(&self) -> Option<SolutionFn> {
        self.solutions.first().map(|(_, func)| *func)
    }

    /// Get a specific solution by name
    pub fn get_solution(&self, name: &str) -> Option<SolutionFn> {
        self.solutions
            .iter()
            .find(|(solution_name, _)| *solution_name == name)
            .map(|(_, func)| *func)
    }

    /// Get all available solution names
    pub fn get_solution_names(&self) -> Vec<&'static str> {
        self.solutions.iter().map(|(name, _)| *name).collect()
    }

    /// Run the primary solution with the given input
    pub fn solve(&self, input: i64) -> Option<i64> {
        self.get_primary_solution().map(|func| func(input))
    }

    /// Run a specific solution by name
    pub fn solve_with(&self, name: &str, input: i64) -> Option<i64> {
        self.get_solution(name).map(|func| func(input))
    }

    /// Validate the solution with the example
    pub fn validate(&self) -> bool {
        match self.solve(self.example_input) {
            Some(result) => result == self.expected_output,
            None => false,
        }
    }

    /// Validate a specific solution by name
    pub fn validate_solution(&self, name: &str) -> bool {
        match self.solve_with(name, self.example_input) {
            Some(result) => result == self.expected_output,
            None => false,
        }
    }
}

/// Registry to hold all problems
pub struct ProblemRegistry {
    problems: HashMap<i64, Problem>,
}

impl ProblemRegistry {
    pub fn new() -> Self {
        ProblemRegistry {
            problems: HashMap::new(),
        }
    }

    pub fn register(mut self, problem: Problem) -> Self {
        self.problems.insert(problem.number, problem);
        self
    }

    pub fn get_problem(&self, number: i64) -> Option<&Problem> {
        self.problems.get(&number)
    }

    pub fn get_all_problems(&self) -> Vec<&Problem> {
        let mut problems: Vec<_> = self.problems.values().collect();
        problems.sort_by_key(|p| p.number);
        problems
    }

    pub fn get_problem_numbers(&self) -> Vec<i64> {
        let mut numbers: Vec<_> = self.problems.keys().cloned().collect();
        numbers.sort();
        numbers
    }
}

/// Initialize and return the problem registry with all problems
pub fn get_problem_registry() -> ProblemRegistry {
    use crate::fibonacci::{sum_of_even_fibonacci_numbers, sum_of_even_fibonacci_numbers_simple};
    use crate::multiples::sum_of_multiples_of_3_and_5;
    use crate::palindromes::find_largest_palindrome;
    use crate::primes::{largest_prime_factor, nth_prime, prime_sum, smallest_multiple};
    use crate::squares::{difference, pythagorean_triplet_product_equal_to};

    ProblemRegistry::new()
        .register(
            Problem::new(
                1,
                "Multiples of 3 and 5",
                "Sum of multiples of 3 or 5 below given number",
                10,
                23,
            )
            .add_solution("formula", sum_of_multiples_of_3_and_5),
        )
        .register(
            Problem::new(
                2,
                "Even Fibonacci Numbers",
                "Sum of even Fibonacci numbers below given number",
                90,
                44, // 2 + 8 + 34 = 44 (even Fibonacci numbers below 90)
            )
            .add_solution("reduced", sum_of_even_fibonacci_numbers)
            .add_solution("simple", sum_of_even_fibonacci_numbers_simple),
        )
        .register(
            Problem::new(
                3,
                "Largest Prime Factor",
                "Largest prime factor of given number",
                13195,
                29,
            )
            .add_solution("factorization", largest_prime_factor),
        )
        .register(
            Problem::new(
                4,
                "Largest Palindrome Product",
                "Largest palindrome from product of two n-digit numbers",
                2,
                9009, // 91 × 99 = 9009
            )
            .add_solution("brute_force", find_largest_palindrome),
        )
        .register(
            Problem::new(
                5,
                "Smallest Multiple",
                "Smallest multiple of all numbers up to given number",
                10,
                2520,
            )
            .add_solution("lcm", smallest_multiple),
        )
        .register(
            Problem::new(
                6,
                "Sum Square Difference",
                "Difference between square of sum and sum of squares",
                10,
                2640, // (1+2+...+10)² - (1²+2²+...+10²) = 55² - 385 = 3025 - 385 = 2640
            )
            .add_solution("formula", difference),
        )
        .register(
            Problem::new(
                7,
                "10001st Prime",
                "Find the nth prime number",
                6,
                13, // 6th prime is 13
            )
            .add_solution("sieve", nth_prime),
        )
        .register(
            Problem::new(
                8,
                "Largest Product in a Series",
                "Largest product of n adjacent digits in a given number",
                4,
                5832,
            )
            .add_solution("brute_force", largest_product_in_series),
        )
        .register(
            Problem::new(
                9,
                "Special Pythagorean Triplet",
                "Product of Pythagorean triplet that sums to given number",
                12,
                60, // 3² + 4² = 5², 3+4+5=12, 3*4*5=60
            )
            .add_solution("search", pythagorean_triplet_product_equal_to),
        )
        .register(
            Problem::new(
                10,
                "Summation of Primes",
                "Sum of all primes below given number",
                10,
                17, // 2 + 3 + 5 + 7 = 17
            )
            .add_solution("sieve", prime_sum),
        )
}
