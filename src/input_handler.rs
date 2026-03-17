use crate::benchmark::{benchmark_all_problems, benchmark_problems, print_benchmark_summary};
use crate::problems::{get_problem_registry, Problem};
use std::{env, fs};
use std::process;

#[derive(Debug, Clone)]
pub struct ProgramArgs {
    pub problem: Problem,
    pub input_value: i64,
    pub solution_method: Option<String>,
}

pub fn parse_arguments() -> ProgramArgs {
    let args: Vec<String> = env::args().collect();

    // Check for help and list flags first
    if args.len() >= 2 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_help(&args[0]);
                process::exit(0);
            }
            "-ls" | "--list" => {
                list_problems();
                process::exit(0);
            }
            "-bench-all" | "--benchmark-all" => {
                println!("Running benchmarks for all problems...");
                println!();
                let results = benchmark_all_problems();
                print_benchmark_summary(&results);
                process::exit(0);
            }
            "-bench" | "--benchmark" => {
                if args.len() < 3 {
                    eprintln!("Error: -bench requires problem numbers");
                    eprintln!(
                        "Usage: {} -bench <problem1> [problem2] [problem3] ...",
                        args[0]
                    );
                    process::exit(1);
                }

                let problem_numbers: Result<Vec<i64>, _> =
                    args[2..].iter().map(|s| s.parse::<i64>()).collect();

                match problem_numbers {
                    Ok(numbers) => {
                        let results = benchmark_problems(&numbers);
                        print_benchmark_summary(&results);
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Error parsing problem numbers: {}", e);
                        process::exit(1);
                    }
                }
            }
            _ => {}
        }
    }

    if args.len() < 3 || args.len() > 4 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let registry = get_problem_registry();
    let problem_number = parse_problem_number(&args[1]);
    let input_value = parse_input_value_to_i64(&args[2]);
    let solution_method = if args.len() == 4 {
        Some(args[3].clone())
    } else {
        None
    };

    let problem = match registry.get_problem(problem_number) {
        Some(p) => p.clone(),
        None => {
            eprintln!("Error: Problem {} is not implemented yet", problem_number);
            eprintln!("Available problems:");
            for num in registry.get_problem_numbers() {
                eprintln!("  {}", num);
            }
            process::exit(1);
        }
    };

    validate_input_value(input_value);

    // Validate solution method if provided
    if let Some(ref method) = solution_method {
        if problem.get_solution(method).is_none() {
            eprintln!(
                "Error: Solution method '{}' not found for problem {}",
                method, problem_number
            );
            eprintln!("Available methods: {:?}", problem.get_solution_names());
            process::exit(1);
        }
    }

    ProgramArgs {
        problem,
        input_value,
        solution_method,
    }
}

fn print_help(program_name: &str) {
    println!("Project Euler Solutions");
    println!("======================");
    println!();
    println!("USAGE:");
    println!(
        "    {} <problem_number> <input_value> [solution_method]",
        program_name
    );
    println!("    {} -bench <problem1> [problem2] ...", program_name);
    println!("    {} -bench-all", program_name);
    println!();
    println!("ARGUMENTS:");
    println!("    <problem_number>    The Project Euler problem number to solve (1-10)");
    println!("    <input_value>       The input value for the problem");
    println!("    [solution_method]   Optional: specific solution method to use");
    println!();
    println!("FLAGS:");
    println!("    -h, --help          Show this help message");
    println!("    -ls, --list         List all available problems with descriptions");
    println!("    -bench              Benchmark specific problems");
    println!("    -bench-all          Benchmark all available problems");
    println!();
    println!("EXAMPLES:");
    println!(
        "    {} 1 1000           # Solve problem 1 with input 1000 (default method)",
        program_name
    );
    println!(
        "    {} 2 90 simple      # Solve problem 2 with input 90 using 'simple' method",
        program_name
    );
    println!(
        "    {} -ls              # List all available problems",
        program_name
    );
    println!(
        "    {} -bench 1 2 3     # Benchmark problems 1, 2, and 3",
        program_name
    );
    println!(
        "    {} -bench-all       # Benchmark all problems",
        program_name
    );
    println!();
    println!(
        "For more information about specific problems, use: {} -ls",
        program_name
    );
}

fn list_problems() {
    println!("Available Project Euler Problems");
    println!("================================");
    println!();

    let registry = get_problem_registry();
    for problem in registry.get_all_problems() {
        println!("Problem {}: {}", problem.number, problem.title);
        println!("  Description: {}", problem.description);
        println!(
            "  Example: input={}, expected={}",
            problem.example_input, problem.expected_output
        );
        println!(
            "  Available methods: {}",
            problem.get_solution_names().join(", ")
        );
        println!();
    }

    println!(
        "Usage: {} <problem_number> <input_value> [solution_method]",
        env::args()
            .next()
            .unwrap_or_else(|| "project_euler".to_string())
    );
}

fn print_usage(program_name: &str) {
    eprintln!(
        "Usage: {} <problem_number> <input_value> [solution_method]",
        program_name
    );
    eprintln!("       {} -h | --help", program_name);
    eprintln!("       {} -ls | --list", program_name);
    eprintln!("       {} -bench <problem1> [problem2] ...", program_name);
    eprintln!("       {} -bench-all", program_name);
    eprintln!();
    eprintln!("Quick overview:");

    let registry = get_problem_registry();
    for problem in registry.get_all_problems() {
        eprintln!(
            "  {} - {} ({})",
            problem.number,
            problem.title,
            problem.get_solution_names().join(", ")
        );
    }
    eprintln!();
    eprintln!("Use -h for detailed help, -ls for full descriptions, or -bench-all for benchmarks.");
}

fn parse_problem_number(arg: &str) -> i64 {
    match arg.parse::<i64>() {
        Ok(num) => num,
        Err(error) => {
            eprintln!("Error: Please provide a valid problem number");
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }
}

fn parse_input_value_to_i64(arg: &str) -> i64 {
    match arg.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Please provide a valid input value");
            process::exit(1);
        }
    }
}

fn validate_input_value(input_value: i64) {
    if input_value <= 0 {
        eprintln!("Error: Please provide a positive input value");
        process::exit(1);
    }
}

pub fn read_grid_from_file(file_path: &str) -> Vec<Vec<i64>> {
    let file = fs::read_to_string(file_path).expect("Failed to read file");
    let lines = file.lines().map(|line| line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect()).collect();
    return lines;
}