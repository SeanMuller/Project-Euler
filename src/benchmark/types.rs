use std::collections::HashMap;
use std::time::Duration;

/// Configuration for benchmarking a specific problem
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub problem_number: i64,
    pub min_input: i64,
    pub max_input: i64,
    pub time_limit_per_solution: Duration,
    pub warmup_iterations: usize,
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

    #[allow(dead_code)] // Utility method for future customization
    pub fn with_warmup(mut self, iterations: usize) -> Self {
        self.warmup_iterations = iterations;
        self
    }
}

/// A single benchmark measurement
#[derive(Debug, Clone)]
pub struct BenchmarkMeasurement {
    pub input_value: i64,
    pub duration: Duration,
    #[allow(dead_code)] // Used for result validation and future analysis
    pub result: i64,
}

/// Results from benchmarking a single solution method
#[derive(Debug, Clone)]
pub struct SolutionBenchmark {
    pub method_name: String,
    #[allow(dead_code)] // Used for detailed analysis and future features
    pub measurements: Vec<BenchmarkMeasurement>,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub iterations_completed: usize,
    pub max_input_reached: i64,
}

/// Results from benchmarking all methods of a problem
#[derive(Debug, Clone)]
pub struct ProblemBenchmark {
    pub problem_number: i64,
    pub problem_title: String,
    #[allow(dead_code)] // Used for detailed reporting and future features
    pub config: BenchmarkConfig,
    pub solution_benchmarks: HashMap<String, SolutionBenchmark>,
}
