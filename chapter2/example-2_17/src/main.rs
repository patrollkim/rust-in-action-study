use std::ops::Add; // std::ops에서 Add 트레이트를 지역 범위로 가져온다.
use std::time::Duration; // std::time에서 Duration 타입을 지역 범위로 가져온다.

fn add<T: Add<Output = T>>(i: T, j: T) -> T { // add()의 인자는 std::ops::Add를 구현하는 어떤 타입도 가능하다.
    i + j
}

fn main() {
    let floats = add(1.2, 3.4); // add() 함수를 부동 소수점 타입을 인자로 하여 호출했다.
    let ints = add(10, 20); // add() 함수를 정수 타입을 인자로 하여 호출했다.
    let durations = add( 
        Duration::new(5, 0),
        Duration::new(10, 0)
    ); // add() 함수를 경과 시간을 나타내는 Duration 타입을 인자로 하여 호출했다.

    println!("{}", floats); 
    println!("{}", ints);
    println!("{:?}", durations); // Duration이 std::fmt::Display 트레이트를 구현하지 않으므로 std::fmt::Debug를 썼다.
}
