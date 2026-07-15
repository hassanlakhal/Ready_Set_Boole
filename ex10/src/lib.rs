fn morton_encode(x: u16, y: u16) -> u32 {
    let mut result: u32 = 0;
    for i in 0..16 {
        let x_bit = ((x >> i) & 1) as u32;
        let y_bit = ((y >> i) & 1) as u32;
        result |= x_bit << (2 * i);
        result |= y_bit << (2 * i + 1);
    }
    result
}


pub fn map(x: u16, y: u16) -> f64{
    

    let combin = morton_encode(x, y);
    

    combin as  f64 / (u32::MAX as f64)
}