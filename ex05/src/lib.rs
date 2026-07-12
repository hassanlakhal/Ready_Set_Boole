use common::ast::parse;
use common::ast::convert_to_nnf;
use common::ast::to_string_rpn;


pub fn negation_normal_form(formula: &str) -> String{
    let tree = parse(formula);
    let nnf_tree = convert_to_nnf(tree);

    let nnf = to_string_rpn(&nnf_tree);
    
    // println!("{}",nnf_tree);

    nnf
}