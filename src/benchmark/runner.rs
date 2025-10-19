use super::config::get_default_benchmark_configs;
use super::types::{BenchmarkConfig, BenchmarkMeasurement, ProblemBenchmark, SolutionBenchmark};
use crate::problems::{get_problem_registry, SolutionFn};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Benchmark a single solution method
pub fn benchmark_solution(
    solution_fn: SolutionFn,
    method_name: &str,
    config: &BenchmarkConfig,
) -> SolutionBenchmark {
    let mut measurements = Vec::new();
    let mut current_input = config.min_input;
    let start_time = Instant::now();

    // Warmup runs
    for _ in 0..config.warmup_iterations {
        let _ = solution_fn(current_input);
    }

    // Actual benchmark runs
    while current_input <= config.max_input && start_time.elapsed() < config.time_limit_per_solution
    {
        let measure_start = Instant::now();
        let result = solution_fn(current_input);
        let duration = measure_start.elapsed();

        measurements.push(BenchmarkMeasurement {
            input_value: current_input,
            duration,
            result,
        });

        // Increment input (adaptive stepping based on current performance)
        if duration < Duration::from_millis(1) {
            current_input += (config.max_input - config.min_input) / 1000; // Large steps for fast functions
        } else if duration < Duration::from_millis(10) {
            current_input += (config.max_input - config.min_input) / 100; // Medium steps
        } else {
            current_input += (config.max_input - config.min_input) / 10; // Small steps for slow functions
        }

        current_input = current_input.min(config.max_input);
    }

    let total_time = start_time.elapsed();
    let iterations_completed = measurements.len();

    // Calculate statistics
    let times: Vec<Duration> = measurements.iter().map(|m| m.duration).collect();
    let average_time = if !times.is_empty() {
        let total_nanos: u128 = times.iter().map(|d| d.as_nanos()).sum();
        let avg_nanos = total_nanos / times.len() as u128;
        Duration::from_nanos(avg_nanos.min(u64::MAX as u128) as u64)
    } else {
        Duration::ZERO
    };

    let min_time = times.iter().min().copied().unwrap_or(Duration::ZERO);
    let max_time = times.iter().max().copied().unwrap_or(Duration::ZERO);
    let max_input_reached = measurements
        .last()
        .map(|m| m.input_value)
        .unwrap_or(config.min_input);

    SolutionBenchmark {
        method_name: method_name.to_string(),
        measurements,
        total_time,
        average_time,
        min_time,
        max_time,
        iterations_completed,
        max_input_reached,
    }
}

/// Benchmark all solution methods for a specific problem
pub fn benchmark_problem(config: BenchmarkConfig) -> Option<ProblemBenchmark> {
    let registry = get_problem_registry();
    let problem = registry.get_problem(config.problem_number)?;

    let mut solution_benchmarks = HashMap::new();

    println!("Benchmarking Problem {}: {}", problem.number, problem.title);
    println!("Input range: {} to {}", config.min_input, config.max_input);
    println!(
        "Time limit per solution: {:.1}s",
        config.time_limit_per_solution.as_secs_f64()
    );
    println!();

    for (method_name, solution_fn) in &problem.solutions {
        print!("  Testing {} method... ", method_name);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let benchmark = benchmark_solution(*solution_fn, method_name, &config);

        println!(
            "completed {} iterations in {:.3}s (avg: {:.6}s per call)",
            benchmark.iterations_completed,
            benchmark.total_time.as_secs_f64(),
            benchmark.average_time.as_secs_f64()
        );

        solution_benchmarks.insert(method_name.to_string(), benchmark);
    }

    Some(ProblemBenchmark {
        problem_number: config.problem_number,
        problem_title: problem.title.to_string(),
        config,
        solution_benchmarks,
    })
}

/// Benchmark all problems with default configurations
pub fn benchmark_all_problems() -> Vec<ProblemBenchmark> {
    let configs = get_default_benchmark_configs();
    let registry = get_problem_registry();
    let mut results = Vec::new();

    println!("Project Euler Solution Benchmarks");
    println!("==================================");
    println!();

    for problem_number in registry.get_problem_numbers() {
        if let Some(config) = configs.get(&problem_number) {
            if let Some(benchmark) = benchmark_problem(config.clone()) {
                results.push(benchmark);
                println!();
            }
        }
    }

    results
}

/// Run benchmarks for specific problems
pub fn benchmark_problems(problem_numbers: &[i64]) -> Vec<ProblemBenchmark> {
    let configs = get_default_benchmark_configs();
    let mut results = Vec::new();

    println!("Benchmarking {} problem(s)", problem_numbers.len());
    println!();

    for &problem_number in problem_numbers {
        if let Some(config) = configs.get(&problem_number) {
            if let Some(benchmark) = benchmark_problem(config.clone()) {
                results.push(benchmark);
                println!();
            }
        } else {
            println!(
                "No benchmark configuration found for problem {}",
                problem_number
            );
        }
    }

    results
}
