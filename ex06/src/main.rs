use ex06::conjunctive_normal_form;

fn main(){

    println!("{}", conjunctive_normal_form("AB&!"));
    // A!B!|
    println!("{}", conjunctive_normal_form("AB|!"));
    // A!B!&
    println!("{}", conjunctive_normal_form("AB|C&"));
    // AB|C&
    println!("{}", conjunctive_normal_form("AB|C|D|"));
    // ABCD|||
    println!("{}", conjunctive_normal_form("AB&C&D&"));
    // ABCD&&&
    println!("{}", conjunctive_normal_form("AB&!C!|"));
    // A!B!C!||
    println!("{}", conjunctive_normal_form("AB|!C!&"));
    // A!B!C!&&
}