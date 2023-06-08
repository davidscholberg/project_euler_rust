use std::fs::read_to_string;

use anyhow::Error;

use crate::project_paths;

/// Get the answer for the given Project Euler problem.
pub fn get(problem_number: u64) -> Result<String, Error> {
    let answers_file_path = project_paths::get_answers_file_path();
    if !answers_file_path.exists() {
        return Err(Error::msg(format!("answers file not found: {}", answers_file_path.display())));
    }
    read_to_string(&answers_file_path)?
        .lines()
        .nth((problem_number + 4) as usize)
        .ok_or(Error::msg(format!("answer to problem number {} not found in answers file {}", problem_number, answers_file_path.display())))
        .map(|answer_str| answer_str.to_string())
}