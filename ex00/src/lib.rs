pub fn adder(a: u32, b: u32) -> u32{
    let mut sum_bit = a ^ b;
    let mut carry_bit = a & b;
    
    while carry_bit != 0 {
        
        let shift = carry_bit << 1;
        carry_bit = sum_bit & shift;
        sum_bit = sum_bit ^ shift;  

    }

    let resule = (carry_bit << 1) | sum_bit;
    resule  
}