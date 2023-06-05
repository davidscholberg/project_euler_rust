use clap::{Parser, Subcommand};

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
    let _cli = Cli::parse();
    println!("Hello, world!");
}
