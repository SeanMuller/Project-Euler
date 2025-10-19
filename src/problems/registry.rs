use super::types::Problem;
use std::collections::HashMap;

/// Registry to hold all problems
pub struct ProblemRegistry {
    problems: HashMap<i64, Problem>,
}

impl ProblemRegistry {
    pub fn new() -> Self {
        ProblemRegistry {
            problems: HashMap::new(),
        }
    }

    pub fn register(mut self, problem: Problem) -> Self {
        self.problems.insert(problem.number, problem);
        self
    }

    pub fn get_problem(&self, number: i64) -> Option<&Problem> {
        self.problems.get(&number)
    }

    pub fn get_all_problems(&self) -> Vec<&Problem> {
        let mut problems: Vec<_> = self.problems.values().collect();
        problems.sort_by_key(|p| p.number);
        problems
    }

    pub fn get_problem_numbers(&self) -> Vec<i64> {
        let mut numbers: Vec<_> = self.problems.keys().cloned().collect();
        numbers.sort();
        numbers
    }
}
