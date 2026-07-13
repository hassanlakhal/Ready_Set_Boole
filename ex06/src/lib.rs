use common::ast::{parse, to_string_rpn, Node, convert_to_nnf};

fn or_right(a: Node, b: Node) -> Node {
    match a {
        Node::Or(c, d) => Node::Or(c, Box::new(or_right(*d, b))),
        _ => Node::Or(Box::new(a), Box::new(b)),
    }
}

fn and_right(a: Node, b: Node) -> Node {
    match a {
        Node::And(c, d) => Node::And(c, Box::new(and_right(*d, b))),
        _ => Node::And(Box::new(a), Box::new(b)),
    }
}

fn convert_to_cnf(node: Node) -> Node {
    let nnf_node = convert_to_nnf(node);
    match nnf_node {

        Node::And(a, b) => and_right(
            convert_to_cnf(*a),
            convert_to_cnf(*b)
        ),

        Node::Or(a, b) => {
            let cnfa = convert_to_cnf(*a);
            let cnfb = convert_to_cnf(*b);
            match (cnfa, cnfb) {

                (Node::And(p, q), other) => convert_to_cnf(
                    Node::And(
                        Box::new(Node::Or(p, Box::new(other.clone()))),
                        Box::new(Node::Or(q, Box::new(other)))
                    )
                ),

                (other, Node::And(p, q)) => convert_to_cnf(
                    Node::And(
                        Box::new(Node::Or(Box::new(other.clone()), p)),
                        Box::new(Node::Or(Box::new(other), q))
                    )
                ),

                (a, b) => or_right(a, b),
            }
        },

        other => other
    }
}

pub fn conjunctive_normal_form(formula: &str) -> String{
    let tree = parse(formula);

    let cnf = convert_to_cnf(tree);

    let rpn = to_string_rpn(&cnf);


    rpn
}