use crate::solvers::factorisation::get_factor_count;

pub fn highly_divisible_triangular_number(n: i64) -> i64 {
    let mut triangle_number: i64 = 0;
    let mut factor_count = 0;
    let mut i = 0;
    while factor_count <= n {
        i+=1;
        triangle_number += i;
        factor_count = get_factor_count(triangle_number);
    }
    triangle_number
}

#[allow(dead_code)]
fn create_triangular_numbers(n: i64) -> Vec<i64> {
    let mut triangular_numbers: Vec<i64> = Vec::with_capacity(n as usize);
    let mut triangle_number: i64 = 0;
    for i in 1..=n {
        triangle_number += i;
        triangular_numbers.push(triangle_number);
    }
    return triangular_numbers;
}
