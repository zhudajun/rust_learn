fn main() {
    test_as();
    test1();
    test3();
    test_5();
    test_from_into();
    test_number();
    test_life_cycle();
}

fn test_as() {
    let decimal = 97.123_f32;
    let integer: u8 = decimal as u8;

    let c1: char = integer as char;
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
//裸指针可以和代表内存地址的整数互相转换
fn test3() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize;
    let second_address: usize = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 指向 values 数组中的第二个元素
    unsafe {
        // 将第二个元素加 1
        *p2 += 1;
    }

    assert_eq!(values[1], 3);

    println!("Success!");
}

// 第 5
fn test_5() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 13);
    }
}

// From 和 Into
fn test_from_into() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // 使用两种方式修复错误
    // 1. 哪个类型实现 From 特征 : impl From<char> for ? ,
    // 你可以查看一下之前提到的文档，来找到合适的类型
    // 2. 上一章节中介绍过的某个关键字
    let i3: i32 = ('a' as u8).into();
    let i4: i32 = i32::from('a' as u8);

    // 使用两种方法来解决错误
    let s: String = String::from('a');
    let s2: String = 'a'.into();

    println!("Success!")
}

// 为自定义类型实现  From trait
#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

fn test_number() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = Number { value: 30 };
    assert_eq!(num.value, 30);
    println!("Success!");
}

// 错误处理 当执行错误处理时，为我们自定义的错误类型实现 From 特征是非常有用。
//这样就可以通过 ? 自动将某个错误类型转换成我们自定义的错误类型

use std::fmt::Error;
use std::fs;
use std::io;
use std::num;
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::IoError(value)
    }
}
impl From<num::ParseIntError> for CliError {
    fn from(value: num::ParseIntError) -> Self {
        CliError::ParseError(value)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? 自动将 io::Error 转换成 CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

// TryFrom 和 TryInto

fn test_try_from_try_into() {
    let n: i16 = 256;
    // Into 特征拥有一个方法 into
    // 因此 TryInto 有一个方法是 ?
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when");
            0
        }
    };
    assert_eq!(n, 0);
}

// 实现 tryFrom trait
#[derive(Debug, PartialEq)]
struct EvenNum(i32);
impl TryFrom<i32> for EvenNum {
    type Error = ();
    // 实现 try_from
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}
fn test_try_from_try_into_() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // 填空
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("Success!")
}

// 生命周期
fn test_life_cycle() {
    let x = "hello";
    let y = "world";
    let result = longest(x, y);
    println!("x = {:?} , y = {:?} , result = {:?}", x, y , result);

    test_life_cycle1();
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_life_cycle1() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("{:?}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("{:?}", borrow2);
    }
}


fn test_life_cycle2() {
   {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
   }
}





#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn test() {}
}
