fn morton_decode(combined: u32) -> (u16, u16) {
    let mut x: u16 = 0;
    let mut y: u16 = 0;

    for i in 0..16 {
        let x_bit = (combined >> (2 * i)) & 1;
        let y_bit = (combined >> (2 * i + 1)) & 1;
        x |= (x_bit as u16) << i;
        y |= (y_bit as u16) << i;
    }

    (x, y)
}

pub fn reverse_map(n: f64) -> (u16, u16) {
    let combined = (n * u32::MAX as f64).round() as u32;
    morton_decode(combined)
}