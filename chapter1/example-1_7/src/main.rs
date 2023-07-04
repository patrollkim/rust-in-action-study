use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10; // stack 영역에 정수 데이터를 생성
    let b = Box::new(20); // heap 영역에 정수 데이터를 생성, 박스된 정수라고도 한다.
    let c = Rc::new(Box::new(30)); // 참조 카운트 안에 박스된 정수를 담는다.
    let d = Arc::new(Mutex::new(40));  
        // 원자적(atomic) 참조 카운터에 담긴 정수이며, 상호 배제(mutual exclusion)잠금 방식으로 보호 받는다.

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
