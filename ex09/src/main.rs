use ex09::eval_set;

fn main(){
   let sets = vec![
        vec![0, 1, 2],
        vec![0, 3, 4],
    ];
    let result = eval_set("AB&", sets);
    println!("{:?}",result);

    // [0]
    let sets = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
    ];
    let result = eval_set("AB|", sets);
     println!("{:?}",result);
    // [0, 1, 2, 3, 4, 5]
    let sets = vec![
        vec![0, 1, 2,4,5],
        vec![0, 1, 2],
        
    ];
    let result = eval_set("AB!", sets);
    println!("{:?}",result);
    // []
}