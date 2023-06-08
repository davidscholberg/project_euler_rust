use anyhow::Error;
use clap::{Parser, Subcommand};

/// Handles retrieving the answer to a Project Euler problem.
mod answer;

/// Gives convenient access to directories within the project.
mod project_paths;

/// Contains each solution.
mod solutions;

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

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Solve { problem_number }) => handle_solve(problem_number),
        None => Err(Error::msg("command not implemented"))
    }
}

/// Runs the solution for the given Project Euler problem number and compares the result to the actual answer.
fn handle_solve(problem_number: u64) -> Result<(), Error> {
    let solutions_hash_map = solutions::get_solutions_hash_map()?;
    let run_solution = solutions_hash_map
        .get(&problem_number)
        .ok_or(Error::msg(format!("no solution found for problem number {}", problem_number)))?;
    let computed_answer = run_solution()?;
    let actual_answer = answer::get(problem_number)?;
    println!("computed answer: {}", computed_answer);
    println!("actual answer:   {}", actual_answer);
    if computed_answer == actual_answer {
        println!("✅ you got it!");
    } else {
        println!("❌ back to the drawing board");
    }
    Ok(())
}