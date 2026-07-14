use ex08::powerset;

fn main(){

    let p = powerset(vec![1,2,3,7]);
    println!("powerset {:?}",p);
    println!("len  {:?}",p.len());
}