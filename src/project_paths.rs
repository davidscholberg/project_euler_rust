use std::path::PathBuf;

/// Get the path to the project root directory.
pub fn get_project_root_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Get the path to the src root directory.
pub fn get_src_root_directory() -> PathBuf {
    let mut src_root = get_project_root_directory();
    src_root.push("src");
    src_root
}

/// Get the path to the solutions directory.
/// 
/// This directory contains the scripts that run the Project Euler solutions.
/// Each script is named with the problem number that the contained solution
/// solves.
pub fn get_solutions_directory() -> PathBuf {
    let mut solutions_directory = get_src_root_directory();
    solutions_directory.push("solutions");
    solutions_directory
}

/// Get the path to the Project Euler solution for the given problem number.
pub fn get_solution_path(problem_number: u64) -> PathBuf {
    let mut solution_path = get_solutions_directory();
    solution_path.push(format!("{:0>4}.rn", problem_number));
    solution_path
}