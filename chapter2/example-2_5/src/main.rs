/*
  - rust 2021 에디션 부터 TryInto 트레이트는 prelude 로 추가됨 
  - 파일 외부의 트레이트는 use 키워드로 지역 범위로 가져와야 사용가능함.
  - unwrap() 메서드를 사용하여 오류 처리 가능하며 해당 메서드는 Result 타입을 반환함.
*/


use std::convert::TryInto; // try_into() 메서드가 구현된 u16과 같은 타입에 해당 메서드를 쓸 수 있도록 함.
// TryInto는 trait 타입. trait는 매서드 집합이라고 할 수도 있음.
// 일반적인 연산 작업(=, +, -, /, * ...)은 암묵적인 prelude로 가져오므로 명시적 가져오기(import, rust에서는 use)는 없어도 됨.

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap(); // try_into() 메서드는 변환 시도의 결과로 Result 타입을 반환함.

    if a < b_ {
        println!("10 is less than 100.");
    }
}
