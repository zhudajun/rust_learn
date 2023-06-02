fn main() {
    test_as();
    test1();
}



fn test_as() {
    let decimal = 97.123_f32;
    let integer: u8 = decimal as u8;

    let c1 : char = integer as char;
    let c2 = integer as char;

    assert_eq!(integer, 'a' as u8);
    println!("success");
}
#[warn(overflowing_literals)]

fn test1() {
    assert_eq!(u8::MAX, 255);
    // 如上所示，u8 类型允许的最大值是 255.
    // 因此以下代码会报溢出的错误： literal out of range for `u8`.
    // **请仔细查看相应的编译错误，从中寻找到解决的办法**
    // **不要修改 main 中的任何代码**
    let v = 1000 as u8;
    println!("Success!")
}
// 3



#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn test() { }
}