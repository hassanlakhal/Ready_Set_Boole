use common::ast::parse;
use common::ast::eval_truth_table;
use std::collections::HashMap;

fn find_variables(formula: &str) -> Vec<char>{

    let mut vars :Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();
    vars.sort();
    vars
}


pub fn print_truth_table(formula: &str){
    let tree = parse(formula);
    let vars = find_variables(formula);
    let n = vars.len();

    print!("| ");
    for var in &vars{
        print!("{} | ", var)
    }
    println!("= |");

    print!("|");
    for _ in &vars{
        print!("---|");
    }
    println!("---|");

    for comob in 0..(1u32 << n){
        let mut vars_map : HashMap<char, bool> = HashMap::new();
        for (i ,v) in vars.iter().enumerate(){
            let bit = (comob >> i) & 1 == 1;
            vars_map.insert(*v, bit);
        }

        let result = eval_truth_table(&tree, &vars_map);
        print!("|");

        for v in &vars{
            print!(" {} |", if vars_map[v] { 1 } else { 0 });
        }
        println!(" {} |" , if result { 1 } else { 0 });
    }
}