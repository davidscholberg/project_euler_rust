use rune::Error;

use crate::{project_paths, wrune};

/// Runs the solution for the given Project Euler problem number and returns
/// the result as a string.
pub fn run(problem_number: u64) -> Result<String, Error> {
    let solution_path = project_paths::get_solution_path(problem_number);
    if !solution_path.exists() {
        return Err(Error::msg(format!("no solution found for problem number {}", problem_number)));
    }
    wrune::call_and_get_string(&solution_path, "add", (10, 20))
}