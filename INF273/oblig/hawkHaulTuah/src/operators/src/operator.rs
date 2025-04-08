use solution::solution::Solution;

pub trait Operator {
    fn name(&self) -> &str;
    fn apply<'a>(&self, solution: &Solution<'a>) -> Solution<'a>;
}