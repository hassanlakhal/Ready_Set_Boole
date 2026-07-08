use ex00::adder;

#[allow(non_snake_case)]
pub fn multiplier(a: u32, b: u32) -> u32{
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