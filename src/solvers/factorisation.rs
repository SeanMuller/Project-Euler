use crate::solvers::primes::prime_factors_grouped;

pub fn get_factor_count(n: i64) -> i64{
    let prime_factors = prime_factors_grouped(n);
    let mut factor_count = 1;
    for (_, count) in prime_factors {
        factor_count *= count +1;
    }
    return factor_count
}