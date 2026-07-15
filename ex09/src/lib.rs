use common::ast::{parse, Node};

fn find_variables(formula: &str) -> Vec<char> {
    let mut vars: Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();
    vars.sort();
    vars.dedup();
    vars
}

fn intersection(set_a: Vec<i32>, set_b: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = set_a
        .into_iter()
        .filter(|x| set_b.contains(x))
        .collect();
    result.sort();
    result.dedup();
    result
}

fn union(set_a: Vec<i32>, set_b: Vec<i32>) -> Vec<i32> {
    let mut result = set_a;
    result.extend(set_b);
    result.sort();
    result.dedup();
    result
}

fn complement(set_a: Vec<i32>, universe: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = universe
        .iter()
        .filter(|x| !set_a.contains(x))
        .cloned()
        .collect();
    result.sort();
    result.dedup();
    result
}

fn logic_set(node: &Node, vars: &[char], sets: &[Vec<i32>], universe: &[i32]) -> Vec<i32> {
    match node {
        Node::Var(c) => {
            let idx = vars.iter().position(|v| v == c).unwrap();
            sets[idx].clone()
        }
        Node::And(a, b) => intersection(
            logic_set(a, vars, sets, universe),
            logic_set(b, vars, sets, universe),
        ),
        Node::Or(a, b) => union(
            logic_set(a, vars, sets, universe),
            logic_set(b, vars, sets, universe),
        ),
        Node::Not(a) => complement(logic_set(a, vars, sets, universe), universe),

        
        _other => unimplemented!("logic_set: unhandled node variant "),
    }
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let vars = find_variables(formula);

    assert_eq!(
        vars.len(),
        sets.len(),
        "number of variables ({}) must match number of sets ({})",
        vars.len(),
        sets.len()
    );

    let mut universe: Vec<i32> = sets.iter().flatten().cloned().collect();
    universe.sort();
    universe.dedup();

    let tree = parse(formula);

    logic_set(&tree, &vars, &sets, &universe)
}