use super::types::ProblemBenchmark;

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

        // Sort methods by iterations completed (descending - more iterations = better performance)
        let mut methods: Vec<_> = benchmark.solution_benchmarks.values().collect();
        methods.sort_by(|a, b| b.iterations_completed.cmp(&a.iterations_completed));

        // Find the best performance (highest iterations completed) for percentage calculation
        let best_iterations = methods.first().map(|m| m.iterations_completed).unwrap_or(1);

        // Find the longest method name for consistent formatting
        let max_name_length = methods
            .iter()
            .map(|m| m.method_name.len())
            .max()
            .unwrap_or(0);

        for (i, method) in methods.iter().enumerate() {
            let rank_symbol = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };

            // Calculate percentage relative to best performance
            let percentage = if best_iterations > 0 {
                (method.iterations_completed as f64 / best_iterations as f64) * 100.0
            } else {
                0.0
            };

            println!(
                "  {} {:<width$} - {:>5.1}% ({:>6}) [min: {:>8.6}s, max: {:>8.6}s]",
                rank_symbol,
                method.method_name,
                percentage,
                method.iterations_completed,
                method.min_time.as_secs_f64(),
                method.max_time.as_secs_f64(),
                width = max_name_length
            );
        }
        println!();
    }
}

/// Print a detailed benchmark report (for future use)
#[allow(dead_code)]
pub fn print_detailed_benchmark_report(benchmarks: &[ProblemBenchmark]) {
    println!("Detailed Benchmark Report");
    println!("=========================");
    println!();

    for benchmark in benchmarks {
        println!(
            "Problem {}: {}",
            benchmark.problem_number, benchmark.problem_title
        );
        println!("Configuration:");
        println!(
            "  Input range: {} to {}",
            benchmark.config.min_input, benchmark.config.max_input
        );
        println!(
            "  Time limit: {:.1}s per solution",
            benchmark.config.time_limit_per_solution.as_secs_f64()
        );
        println!(
            "  Warmup iterations: {}",
            benchmark.config.warmup_iterations
        );
        println!();

        // Sort methods by iterations completed (descending - more iterations = better performance)
        let mut methods: Vec<_> = benchmark.solution_benchmarks.values().collect();
        methods.sort_by(|a, b| b.iterations_completed.cmp(&a.iterations_completed));

        // Find the best performance (highest iterations completed) for percentage calculation
        let best_iterations = methods.first().map(|m| m.iterations_completed).unwrap_or(1);

        for method in &methods {
            // Calculate percentage relative to best performance
            let percentage = if best_iterations > 0 {
                (method.iterations_completed as f64 / best_iterations as f64) * 100.0
            } else {
                0.0
            };

            println!("  Method: {} ({:.1}%)", method.method_name, percentage);
            println!("    Total time: {:.3}s", method.total_time.as_secs_f64());
            println!(
                "    Average time: {:.6}s",
                method.average_time.as_secs_f64()
            );
            println!("    Min time: {:.6}s", method.min_time.as_secs_f64());
            println!("    Max time: {:.6}s", method.max_time.as_secs_f64());
            println!("    Iterations: {}", method.iterations_completed);
            println!("    Max input reached: {}", method.max_input_reached);
            println!();
        }

        println!("---");
        println!();
    }
}

/// Export benchmark results to CSV format (for future use)
#[allow(dead_code)]
pub fn export_to_csv(benchmarks: &[ProblemBenchmark]) -> String {
    let mut csv = String::new();
    csv.push_str("problem_number,problem_title,method_name,avg_time_ms,min_time_ms,max_time_ms,iterations,max_input,performance_percentage\n");

    for benchmark in benchmarks {
        // Sort methods by iterations completed to calculate percentages
        let mut methods: Vec<_> = benchmark.solution_benchmarks.values().collect();
        methods.sort_by(|a, b| b.iterations_completed.cmp(&a.iterations_completed));

        // Find the best performance for percentage calculation
        let best_iterations = methods.first().map(|m| m.iterations_completed).unwrap_or(1);

        for method in &methods {
            // Calculate percentage relative to best performance
            let percentage = if best_iterations > 0 {
                (method.iterations_completed as f64 / best_iterations as f64) * 100.0
            } else {
                0.0
            };

            csv.push_str(&format!(
                "{},{},{},{:.6},{:.6},{:.6},{},{},{:.1}\n",
                benchmark.problem_number,
                benchmark.problem_title.replace(',', ";"), // Escape commas
                method.method_name,
                method.average_time.as_secs_f64() * 1000.0,
                method.min_time.as_secs_f64() * 1000.0,
                method.max_time.as_secs_f64() * 1000.0,
                method.iterations_completed,
                method.max_input_reached,
                percentage
            ));
        }
    }

    csv
}
