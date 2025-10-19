/// Type alias for solution functions
pub type SolutionFn = fn(i64) -> i64;

/// Represents a Project Euler problem with metadata and multiple solution approaches
#[derive(Clone, Debug)]
pub struct Problem {
    pub number: i64,
    pub title: &'static str,
    pub description: &'static str,
    pub example_input: i64,
    pub expected_output: i64,
    pub solutions: Vec<(&'static str, SolutionFn)>,
}

impl Problem {
    /// Create a new problem instance
    pub fn new(
        number: i64,
        title: &'static str,
        description: &'static str,
        example_input: i64,
        expected_output: i64,
    ) -> Self {
        Problem {
            number,
            title,
            description,
            example_input,
            expected_output,
            solutions: Vec::new(),
        }
    }

    /// Add a solution method to this problem
    pub fn add_solution(mut self, name: &'static str, solution_fn: SolutionFn) -> Self {
        self.solutions.push((name, solution_fn));
        self
    }

    /// Get the primary (first) solution
    pub fn get_primary_solution(&self) -> Option<SolutionFn> {
        self.solutions.first().map(|(_, func)| *func)
    }

    /// Get a specific solution by name
    pub fn get_solution(&self, name: &str) -> Option<SolutionFn> {
        self.solutions
            .iter()
            .find(|(solution_name, _)| *solution_name == name)
            .map(|(_, func)| *func)
    }

    /// Get all available solution names
    pub fn get_solution_names(&self) -> Vec<&'static str> {
        self.solutions.iter().map(|(name, _)| *name).collect()
    }

    /// Run the primary solution with the given input
    pub fn solve(&self, input: i64) -> Option<i64> {
        self.get_primary_solution().map(|func| func(input))
    }

    /// Run a specific solution by name
    pub fn solve_with(&self, name: &str, input: i64) -> Option<i64> {
        self.get_solution(name).map(|func| func(input))
    }

    /// Validate the solution with the example
    pub fn validate(&self) -> bool {
        match self.solve(self.example_input) {
            Some(result) => result == self.expected_output,
            None => false,
        }
    }

    /// Validate a specific solution by name
    pub fn validate_solution(&self, name: &str) -> bool {
        match self.solve_with(name, self.example_input) {
            Some(result) => result == self.expected_output,
            None => false,
        }
    }
}
