// Module declarations
pub mod config;
pub mod reporter;
pub mod runner;
pub mod types;

// Re-export only the functions that are used externally
pub use reporter::print_benchmark_summary;
pub use runner::{benchmark_all_problems, benchmark_problems};
