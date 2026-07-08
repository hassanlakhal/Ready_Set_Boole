
fn adder(a: u32, b: u32) -> u32{
    let sum_bit = a ^ b;
    let carry_bit = a & b;

    let resule = (carry_bit << 1) | sum_bit;
    resule  
}

#[allow(non_snake_case)]
fn multiplier(a: u32, b: u32) -> u32{
    let mut resule = 0;
    let mut A = a;
    let mut B = b;
    for  _ in  0..32 {
        if (B & 1) == 1 {
            resule = adder(resule, A);
        }
        A = A << 1;
        B = B >> 1;
    }
    resule 
}


fn main(){
    let mult = multiplier(4545,12);
    println!("{}", mult)
}