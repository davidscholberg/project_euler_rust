use anyhow::Error;

use super::Solution;

pub static SOLUTION: Solution = Solution{
    number: 1,
    run,
};

fn run() -> Result<String, Error> {
    Ok(String::from("hai solution 1 here"))
}