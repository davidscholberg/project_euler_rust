use std::path::PathBuf;

/// Get the path to the answers file.
pub fn get_answers_file_path() -> PathBuf {
    let mut answers_file_path = get_data_root_directory();
    answers_file_path.push("answers/answers.txt");
    answers_file_path
}

/// Get the path to the data root directory.
pub fn get_data_root_directory() -> PathBuf {
    let mut data_root_directory = get_project_root_directory();
    data_root_directory.push("data");
    data_root_directory
}

/// Get the path to the project root directory.
pub fn get_project_root_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}