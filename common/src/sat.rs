use std::collections::HashMap;

pub enum Formul{
    Var(char),
    Not(Box<Formul>),
    And(Box<Formul>, Box<Formul>),
    Or(Box<Formul>, Box<Formul>)
}

pub fn parse_sat(formula: &str) -> Formul{

    let mut stack : Vec<Formul> = Vec::new();


    for c in formula.chars(){
        match c {
            'A'..='Z'=> stack.push(Formul::Var(c)),
            '!' => {
                let a = stack.pop().unwrap();
                stack.push(Formul::Not(Box::new(a)));
            }
            '&' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Formul::And(Box::new(a), Box::new(b)));
            }
            '|' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Formul::Or(Box::new(a), Box::new(b)));
            }
            _ => println!("invalid formula")
        }
    }
    stack.pop().unwrap()
}

pub fn eval_sat(formul: &Formul, vars: &HashMap<char, bool>) ->bool{
    match formul{
        Formul::Var(a) => *vars.get(a).expect("unbound variabl"),
        Formul::And(a, b) => eval_sat(a, vars) && eval_sat(b, vars),
        Formul::Or(a, b) => eval_sat(a, vars) || eval_sat(b, vars),
        Formul::Not(a) => !eval_sat(a, vars)
    }
} 