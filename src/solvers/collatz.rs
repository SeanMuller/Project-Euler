use std::collections::hash_map;

fn collatz (n: i64) -> Vec<i64>{
    let mut sequence: Vec<i64> = Vec::with_capacity(1000);
    let mut i = n;
    while i != 1{
        sequence.push(i);
        if i % 2 == 0{    
            i /=2;
        }else{
            i = i*3 +1
        }
    }
    sequence.push(1);
    return sequence;
}

fn collatz_optimised(n: i64) -> i64{
    let 
}


pub fn longest_collatz(n:i64) -> i64{
    let mut max_sequence = 0;
    let mut starting_number = 0;
    for i in 1..n {
        let sequence = collatz(i);
        if sequence.len() > max_sequence{
            max_sequence = sequence.len();
            starting_number = i
        }
    } 
    return starting_number;
}
