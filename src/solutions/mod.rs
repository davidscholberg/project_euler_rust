use std::collections::HashMap;

use anyhow::Error;

mod s0001;

/// Type for functions that run Project Euler solutions.
type RunFunction = fn() -> Result<String, Error>;

/// Contains the problem number and solution function of a Project Euler solution.
pub struct Solution {
    /// The Project Euler problem number.
    pub number: u64,
    // /// The function that runs this problem's solution.
    pub run: RunFunction,
}

/// Type for problem number => run function hash map.
type HashMapResult = Result<HashMap<u64, RunFunction>, Error>;

/// Get the solutions hash map which maps problem numbers to their corresponding solution functions.
pub fn get_solutions_hash_map() -> HashMapResult {
    let solutions = [
        &s0001::SOLUTION,
    ];
    let mut solutions_hash_map = HashMap::new();
    for solution in solutions {
        if solutions_hash_map.insert(solution.number, solution.run).is_some() {
            return Err(Error::msg(format!("duplicate solution number {}", solution.number)))
        }
    }
    Ok(solutions_hash_map)
}