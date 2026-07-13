use ex07::sat;

fn main(){
    println!("{}", sat("AB|"));
    // true
    println!("{}", sat("AB&"));
    // true
    println!("{}", sat("AA!&"));
    // false
    println!("{}", sat("AA^"));
    // false
      println!("{}", sat("QW^"));
    // True
    }