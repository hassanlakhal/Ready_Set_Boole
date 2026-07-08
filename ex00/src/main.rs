fn adder(a: u32, b: u32) -> u32{
    let sum_bit = a ^ b;
    let carry_bit = a & b;

    let resule = (carry_bit << 1) | sum_bit;
    resule  
}

fn main() {
    let sum = adder(1,1336);
    println!("{}",sum);
}
