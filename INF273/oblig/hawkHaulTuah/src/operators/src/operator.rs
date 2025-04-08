use solution::solution::Solution;

pub trait Operator {
    fn name(&self) -> &str;
    fn apply(&self, solution: &Solution) -> Solution;
}