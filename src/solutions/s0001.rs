use std::{iter::repeat_with, collections::HashSet};

use anyhow::Error;

use super::Solution;

pub static SOLUTION: Solution = Solution{
    number: 1,
    run,
};

fn run() -> Result<String, Error> {
    let multiples_of_3 = repeat_with(make_next_multiple(3)).take_while(|n| *n < 1000);
    let multiples_of_5 = repeat_with(make_next_multiple(5)).take_while(|n| *n < 1000);
    let mut multiples_hash_set = HashSet::new();
    multiples_hash_set.extend(multiples_of_3.chain(multiples_of_5));
    let answer: u64 = multiples_hash_set.iter().sum();
    Ok(answer.to_string())
}

/// Returns closure that repeatedly returns multiples of the given number.
fn make_next_multiple(of: u64) -> impl FnMut() -> u64 {
    let mut current_multiple = 0;
    move || {
        current_multiple += of;
        current_multiple
    }
}