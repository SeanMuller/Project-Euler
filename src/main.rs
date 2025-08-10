mod benchmark;
mod fibonacci;
mod input_handler;
mod multiples;
mod palindromes;
mod primes;
mod problems;
mod series;
mod squares;

use input_handler::parse_arguments;

fn main() {
    let args = parse_arguments();

    // Determine which solution method to use
    let result = match &args.solution_method {
        Some(method) => match args.problem.solve_with(method, args.input_value) {
            Some(result) => result,
            None => {
                eprintln!("Error: Failed to execute solution method '{}'", method);
                std::process::exit(1);
            }
        },
        None => match args.problem.solve(args.input_value) {
            Some(result) => result,
            None => {
                eprintln!(
                    "Error: No solution available for problem {}",
                    args.problem.number
                );
                std::process::exit(1);
            }
        },
    };

    // Display the result - use first solution name as default
    let method_name = args.solution_method.clone().unwrap_or_else(|| {
        args.problem
            .get_solution_names()
            .first()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    });

    println!(
        "Problem {} - {} (using {}): {}",
        args.problem.number, args.problem.title, method_name, result
    );

    // Optionally show validation result for the example
    if args.input_value == args.problem.example_input {
        let is_valid = match &args.solution_method {
            Some(method) => args.problem.validate_solution(method),
            None => args.problem.validate(),
        };

        if is_valid {
            println!("✓ Solution validated with example input");
        } else {
            println!("✗ Solution validation failed with example input");
        }
    }
}
