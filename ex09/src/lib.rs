use common::ast::{parse, Node};

fn find_variables(formula: &str) -> Vec<char>{

    let mut vars :Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();
    vars.sort();
    vars.dedup();
    vars
}

fn intersection(set_a: Vec<i32>, set_b: Vec<i32>) -> Vec<i32>{

    let mut result: Vec<i32> = vec![];

    for i in 0..set_a.len(){
        if set_b.contains(&set_a[i]){
            result.push(set_a[i])
        }
    }

    result
}

fn union(set_a: Vec<i32>, set_b: Vec<i32>) -> Vec<i32>{
    let mut result: Vec<i32> = vec![];

    for i in 0..set_a.len(){

        result.push(set_a[i]);
        for j in 0..set_b.len(){
            if result.contains(&set_b[j]){
                continue ;
            }
            else {
                result.push(set_b[j]);
            }
        }
    }
    result
}

fn logic_set(node: &Node, vars: &[char], sets: &[Vec<i32>]) -> Vec<i32>{
    println!("Node : {}", node);
    match node {
        Node::Var(c) => {
            let idx = vars.iter().position(|v| v == c).unwrap();
            sets[idx].clone()
        }
        Node::And(a, b) => {
          intersection(
                logic_set(a, vars, sets), 
                logic_set(b, vars, sets)
            )  
        }

        Node::Or(a, b) => {
            union(
                logic_set(a, vars, sets), 
                logic_set(b, vars, sets)
            )
        }
        _other => vec![]
    }

}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>{

    let vars = find_variables(formula);
    let n = vars.len();

    let tree = parse(formula);


    let result = logic_set(&tree,&vars, &sets);

    result


}