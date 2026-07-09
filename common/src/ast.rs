use std::collections::HashMap;

pub enum Node {
    Var(char),
    Lit(bool),
    Not(Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Xor(Box<Node>, Box<Node>),
    Imply(Box<Node>, Box<Node>),
    Equiv(Box<Node>, Box<Node>),
}

pub fn parse(formula: &str) -> Node{

    let mut stack : Vec<Node> = Vec::new();

    for c in formula.chars(){
        match c {
            'A'..='Z'=> stack.push(Node::Var(c)),
            '0' => stack.push(Node::Lit(false)),
            '1' => stack.push(Node::Lit(true)),
            '!' => {
                let a = stack.pop().unwrap();
                stack.push(Node::Not(Box::new(a)));
            },
            '&' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Node::And(Box::new(a), Box::new(b)));
            },
            '|' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Node::Or(Box::new(a), Box::new(b)));
            },
            '^' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Node::Xor(Box::new(a), Box::new(b)));
            }
            '=' => {
                 let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Node::Equiv(Box::new(a), Box::new(b)));
            }
            '>' => {
                 let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Node::Equiv(Box::new(a), Box::new(b)));
            }
            _ => println!("invalid formula")
        }
    }

    stack.pop().unwrap()
}

pub fn eval(node: &Node) -> bool {
    match node {
        Node::Lit(a) => *a,
        Node::Not(a) => !eval(a),
        Node::And(a, b) => eval(a) &&  eval(b),
        Node::Or(a, b) => eval(a) || eval(b),
        Node::Xor(a, b) => eval(a) ^ eval(b),
        Node::Imply(a, b) => !eval(a) || eval(b),
        Node::Equiv(a, b) => eval(a) == eval(b),
        Node::Var(c) => panic!("Cannot evaluate variable '{}' without an environment mapping!", c),  
    }
}

pub fn eval_truth_table(node: &Node, vars: &HashMap<char, bool>) -> bool {
    match node {
        Node::Lit(b) => *b,
        Node::Var(c) => *vars.get(c).expect("unbound variable"),
        Node::Not(a) => !eval_truth_table(a, vars),
        Node::And(a, b) => eval_truth_table(a, vars) && eval_truth_table(b, vars),
        Node::Or(a, b) => eval_truth_table(a, vars) || eval_truth_table(b, vars),
        Node::Xor(a, b) => eval_truth_table(a, vars) ^ eval_truth_table(b, vars),
        Node::Imply(a, b) => !eval_truth_table(a, vars) || eval_truth_table(b, vars),
        Node::Equiv(a, b) => eval_truth_table(a, vars) == eval_truth_table(b, vars),
    }
}