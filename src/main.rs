use clap::{Parser, Subcommand};

/// Gives convenient access to directories within the project.
mod project_paths;
/// Wrapper for Rune. Simplifies interfacing with rune.
mod wrune;
/// Handles running a Project Euler solution and returning the result.
mod solution;

/// Program for running Project Euler solutions.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the solution to a given Project Euler problem.
    Solve {
        /// The problem number to run the solution for.
        problem_number: u64,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Solve { problem_number }) => {
            match solution::run(problem_number) {
                Ok(output) => println!("great success: {}", output),
                Err(err) => eprintln!("oopsie: {}", err),
            }
        },
        None => {}
    }
}
