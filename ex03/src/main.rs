use ex03::eval_formula;


fn main(){

    println!("{}", eval_formula("10&"));
    // false
    println!("{}", eval_formula("10|"));
    // true
    println!("{}", eval_formula("11>"));
    // true
    println!("{}", eval_formula("10="));
    // false
    println!("{}", eval_formula("1011||="));
    // true
}