use common::ast::parse;
use common::ast::eval;

pub fn eval_formula(formula: &str) -> bool{
    let tree = parse(formula);
    eval(&tree)
}