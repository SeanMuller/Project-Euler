use crate::problems::{get_problem_registry, SolutionFn};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Configuration for benchmarking a specific problem
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub problem_number: i64,
    pub min_input: i64,
    pub max_input: i64,
    pub time_limit_per_solution: Duration,
    pub warmup_iterations: usize,
}

/// Results from benchmarking a single solution method
#[derive(Debug, Clone)]
pub struct SolutionBenchmark {
    pub method_name: String,
    pub measurements: Vec<BenchmarkMeasurement>,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub iterations_completed: usize,
    pub max_input_reached: i64,
}

/// A single benchmark measurement
#[derive(Debug, Clone)]
pub struct BenchmarkMeasurement {
    pub input_value: i64,
    pub duration: Duration,
    pub result: i64,
}

/// Results from benchmarking all methods of a problem
#[derive(Debug, Clone)]
pub struct ProblemBenchmark {
    pub problem_number: i64,
    pub problem_title: String,
    pub config: BenchmarkConfig,
    pub solution_benchmarks: HashMap<String, SolutionBenchmark>,
}

impl BenchmarkConfig {
    pub fn new(
        problem_number: i64,
        min_input: i64,
        max_input: i64,
        time_limit_seconds: f64,
    ) -> Self {
        Self {
            problem_number,
            min_input,
            max_input,
            time_limit_per_solution: Duration::from_secs_f64(time_limit_seconds),
            warmup_iterations: 3,
        }
    }

    pub fn with_warmup(mut self, iterations: usize) -> Self {
        self.warmup_iterations = iterations;
        self
    }
}

/// Default benchmark configurations for each problem
pub fn get_default_benchmark_configs() -> HashMap<i64, BenchmarkConfig> {
    let mut configs = HashMap::new();

    // Problem 1: Multiples of 3 and 5
    configs.insert(1, BenchmarkConfig::new(1, 10, 1000000, 5.0));

    // Problem 2: Even Fibonacci Numbers
    configs.insert(2, BenchmarkConfig::new(2, 90, 40000000, 5.0));

    // Problem 3: Largest Prime Factor
    configs.insert(3, BenchmarkConfig::new(3, 13195, 600851475143, 10.0));

    // Problem 4: Largest Palindrome Product
    configs.insert(4, BenchmarkConfig::new(4, 2, 4, 15.0)); // 2-digit to 4-digit

    // Problem 5: Smallest Multiple
    configs.insert(5, BenchmarkConfig::new(5, 10, 30, 10.0));

    // Problem 6: Sum Square Difference
    configs.insert(6, BenchmarkConfig::new(6, 10, 10000, 5.0));

    // Problem 7: Nth Prime
    configs.insert(7, BenchmarkConfig::new(7, 6, 10001, 15.0));

    // Problem 8: Largest Product in Series
    configs.insert(8, BenchmarkConfig::new(8, 4, 13, 5.0));

    // Problem 9: Pythagorean Triplet
    configs.insert(9, BenchmarkConfig::new(9, 12, 1000, 10.0));

    // Problem 10: Sum of Primes
    configs.insert(10, BenchmarkConfig::new(10, 10, 2000000, 20.0));

    configs
}

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

/// Print a summary report of benchmark results
pub fn print_benchmark_summary(benchmarks: &[ProblemBenchmark]) {
    println!("Benchmark Summary");
    println!("=================");
    println!();

    for benchmark in benchmarks {
        println!(
            "Problem {}: {}",
            benchmark.problem_number, benchmark.problem_title
        );

        // Sort methods by average performance
        let mut methods: Vec<_> = benchmark.solution_benchmarks.values().collect();
        methods.sort_by_key(|b| b.average_time);

        for (i, method) in methods.iter().enumerate() {
            let rank_symbol = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };

            println!(
                "  {} {} - avg: {:.6}s, max input: {}, iterations: {}",
                rank_symbol,
                method.method_name,
                method.average_time.as_secs_f64(),
                method.max_input_reached,
                method.iterations_completed
            );
        }
        println!();
    }
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
