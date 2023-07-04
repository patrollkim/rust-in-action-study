fn main() {
    let num1 = 1;
    let num2 = 2;

    println!("{} + {} = {}", num1, num2, add(num1, num2));
}

// 여기서 T는 + 연산을 지원하지 않는 모든 타입을 의미하기 때문에 컴파일 할 수 없다.
fn add<T>(i: T, j: T) -> T {
    i + j // + 연산자는 std::ops::Add를 구현하는 타입만 가능하다.
}
