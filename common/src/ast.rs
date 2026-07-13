use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
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
                stack.push(Node::And(Box::new(b), Box::new(a)));
            },
            '|' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Node::Or(Box::new(b), Box::new(a)));
            },
            '^' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Node::Xor(Box::new(b), Box::new(a)));
            }
            '=' => {
                 let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Node::Equiv(Box::new(b), Box::new(a)));
            }
            '>' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Node::Imply(Box::new(b), Box::new(a)));
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

pub fn convert_to_nnf(node: Node) -> Node {
    match node {
        Node::Var(c) => Node::Var(c),

        Node::Not(inner) => match *inner{
            Node::Not(x) => convert_to_nnf(*x),
            Node::And(a, b) => Node::Or(
                Box::new(convert_to_nnf(Node::Not(a))),
                Box::new(convert_to_nnf(Node::Not(b)))
            ),
            Node::Or(a, b) => Node::And(
                Box::new(convert_to_nnf(Node::Not(a))),
                Box::new(convert_to_nnf(Node::Not(b)))
            ),

            other => Node::Not(Box::new(convert_to_nnf(other)))
        },

        Node::And(a, b) => Node::And(
            Box::new(convert_to_nnf(*a)),
            Box::new(convert_to_nnf(*b))
        ),

        Node::Or(a, b) => Node::Or(
            Box::new(convert_to_nnf(*a)),
            Box::new(convert_to_nnf(*b))
        ),
        // A ↔ B  ≡  (A ∧ B) ∨ (¬A ∧ ¬B)
        Node::Equiv(a, b) => {
            convert_to_nnf(
                Node::Or(
                    Box::new(Node::And(a.clone(), b.clone())),
                    Box::new(Node::And(
                        Box::new(Node::Not(a.clone())), 
                        Box::new(Node::Not(b.clone()))
                    ))
                ))
        },
        // A → B  ≡  ¬A ∨ B
        Node::Imply(a, b) => {
            convert_to_nnf(
                Node::Or(
                    Box::new(Node::Not(a)),
                    b,
                )
            )
        }
        Node::Xor(a,b) => {
            convert_to_nnf(Node::Or(
                Box::new(Node::And(a.clone(), Box::new(Node::Not(b.clone())))),
                Box::new(Node::And(Box::new(Node::Not(a)), b))
            ))
        }
        _ => node
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Var(c) => write!(f, "{}", c),
            Node::Lit(n) => write!(f, "{}", n),
            Node::Not(inner) => write!(f, "{}!", inner),
            Node::And(a, b) => write!(f, "{}{}&", a, b),
            Node::Or(a, b) => write!(f, "{}{}|", a, b),
            Node::Xor(a, b) => write!(f, "{}{}^",a, b),
            Node::Imply(a, b) => write!(f, "{}{}>", a, b),
            Node::Equiv(a, b) => write!(f, "{}{}=" , a, b)
        }
    }
}

pub fn to_string_rpn(node: &Node) -> String {
    match node {
        Node::Var(c) => c.to_string(),
        Node::Lit(n) => n.to_string(),
        Node::Not(inner) => format!("{}!", to_string_rpn(inner)),
        Node::And(a, b) => format!("{}{}&", to_string_rpn(a), to_string_rpn(b)),
        Node::Or(a, b) => format!("{}{}|", to_string_rpn(a), to_string_rpn(b)),
        Node::Xor(a, b) => format!("{}{}^", to_string_rpn(a), to_string_rpn(b)),
        Node::Imply(a, b) => format!("{}{}>", to_string_rpn(a), to_string_rpn(b)),
        Node::Equiv(a, b) => format!("{}{}=", to_string_rpn(a), to_string_rpn(b)),
    }
}