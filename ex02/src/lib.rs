
pub fn gray_code(n: u32) -> u32{

    let gray =  n ^ (n >> 1);

    return gray;
}