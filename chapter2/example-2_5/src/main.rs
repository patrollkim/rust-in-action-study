//use std::convert::TryInto; => rust 2021 에디션 부터 prelude 로 추가됨 

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("10 is less than 100.");
    }
}
