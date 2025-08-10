/// Functions for calculating sums of multiples
/// Used primarily for Project Euler Problem 1

/// Calculate the sum of multiples of a given number below n
/// Uses the arithmetic series formula: sum = count * (count + 1) * multiple / 2
pub fn sum_of_multiples(n: i64, multiple: i64) -> i64 {
    let a = (n - 1) / multiple;
    return a * (a + 1) * multiple / 2;
}

/// Calculate the sum of all multiples of 3 or 5 below n
/// Uses inclusion-exclusion principle to avoid double counting multiples of 15
pub fn sum_of_multiples_of_3_and_5(n: i64) -> i64 {
    let sum_of_3s = sum_of_multiples(n, 3);
    let sum_of_5s = sum_of_multiples(n, 5);
    let sum_of_15s = sum_of_multiples(n, 15);
    return sum_of_3s + sum_of_5s - sum_of_15s;
}
