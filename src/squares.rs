use std::collections::HashSet;

pub fn sum_of_squares(n: i64) -> i64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i.pow(2);
    }
    return sum;
}

pub fn square_of_sum(n: i64) -> i64 {
    let sum = (n * (n + 1)) / 2;
    return sum.pow(2);
}

pub fn difference(n: i64) -> i64 {
    return square_of_sum(n) - sum_of_squares(n);
}

// pub fn squares_less_than(n: i64) -> HashSet<i64> {
//     let mut squares:HashSet<i64> = HashSet::new();
//     for i in 1..=n.isqrt() {
//         squares.insert(i.pow(2));
//     }
//     return squares;
// }

// pub fn sum_of_squares_equal_to(n: i64) -> HashSet<i64> {
//     let mut sum_of_squares:HashSet<i64> = HashSet::new();
//     let squares = squares_less_than(n);
//     for i in squares.iter() {
//         for j in squares.iter() {
//             if squares.contains(&(n - *i - *j)) {
//                 sum_of_squares.insert(*i);
//                 sum_of_squares.insert(*j);
//                 sum_of_squares.insert(n - *i - *j);
//             }
//         }
//     }
//     return sum_of_squares;
// }

pub fn pythagorean_triplet_sum_to(n: i64) -> HashSet<i64> {
    let mut pythagorean_triplet:HashSet<i64> = HashSet::new();
    for a in 1..=n   {
        for b in a..=n {
            let c = n - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == n {
                pythagorean_triplet.insert(a);
                pythagorean_triplet.insert(b);
                pythagorean_triplet.insert(c);
                return pythagorean_triplet;
            }
        }
    }
    return pythagorean_triplet;
}

pub fn pythagorean_triplet_product_equal_to(n: i64) -> i64 {
    let pythagorean_triplet = pythagorean_triplet_sum_to(n);
    return pythagorean_triplet.iter().product();
}