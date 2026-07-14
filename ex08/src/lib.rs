pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>>{
    let mut result : Vec<Vec<i32>> = vec![];

    let n = set.len();

    let comob = 1u32 << n;

    result.push(vec![]);

    for i in 1..(comob -1){
        let mut sub : Vec<i32> =  Vec::new();
        for pos in 0..n{
            if (i >> pos) & 1 == 1{
                sub.push(set[pos]);
            }
        }
        result.push(sub) 
    }

    result.push(set);

    result
}