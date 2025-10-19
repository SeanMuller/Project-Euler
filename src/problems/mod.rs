// Module declarations
pub mod definitions;
pub mod registry;
pub mod types;

// Re-export the main types and functions for easy access
pub use definitions::get_problem_registry;
pub use types::{Problem, SolutionFn};
