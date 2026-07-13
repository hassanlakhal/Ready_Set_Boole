use common::sat::{parse_sat, eval_sat};
use common::ast::{parse, to_string_rpn, convert_to_nnf};
 use std::collections::HashMap;

fn find_variables(formula: &str) -> Vec<char>{

    let mut vars :Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();
    vars.sort();
    vars.dedup();
    vars
}

pub fn sat(formula: &str) -> bool{

    let tree = parse(formula);

    let nnf = convert_to_nnf(tree);
    let rpn = to_string_rpn(&nnf);

    let vars = find_variables(&rpn);
    let tree_sat = parse_sat(&rpn);
    let n = vars.len();

    for comob in 0..(1u32 << n){
        let mut vars_map : HashMap<char, bool> = HashMap::new();
        for (i, v) in vars.iter().enumerate(){
            let bit = (comob >> i) & 1 == 1;
            vars_map.insert(*v, bit);
        }
        if eval_sat(&tree_sat, &vars_map){
           return true;
        }
    }
    false
}