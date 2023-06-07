use std::error::Error;

fn main() {
    //drink("lemonade");
    normal_panic();
}

// 实现一个 panic
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Sucess");
        panic!("crash and burn");
    }
    println!("打印这句话就失败了");
}

// 常见的 panic 例子
fn normal_panic() {
    assert_eq!("abc".as_bytes(), [97, 98, 99]);
    let v = vec![1, 2, 3];
    let ele = v[2];
    let ele = v.get(2).unwrap();
    println!("{:?}", ele);
    let v = proudction_rate_per_hour(2);
    divide(15, 1);
    println!("succeeded");
}

fn divide(x: u8, y: u8) {
    println!("{}", x / y);
}

fn proudction_rate_per_hour(speed: u16) -> f64 {
    let cph: u16 = 221;

    let result = result_test("10", "2");
    assert_eq!(result.unwrap(), 20);
    let result = result_test("4", "2");
    assert_eq!(result.unwrap(), 8);

    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }

}

use std::num::ParseIntError;
// Result and ?
fn result_test(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    println!("{:?} ---- {:?}", n1 , n2);
    Ok(n1.unwrap() * n2.unwrap())
}
// ? 强制获取  成功之后的值
fn multiply(n1_str:&str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>() ?;
    let n2 = n2_str.parse::<i32>() ?;
   Ok(n1 * n2)
}
use std::fs::File;
use std::io::{self, Read};

// 读文档的内容
fn read_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return  Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// ? 的使用,可以连续的使用
fn read_file1() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


// map & and_then 常用的组合器









#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        println!("测试中....");
    }
}
