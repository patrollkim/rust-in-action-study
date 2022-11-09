/*
    러스트에서 사용 가능한 정수 타입 : i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
      ==> i는 부호 있는 정수, u는 부호 없는 정수, i/u128의 경우 FFI와 clang 컴파일러와 호환성 이슈가 있음.
      ==> i/usize의 경우 CPU 아키텍처에 따라 크기가 정해짐
    러스트에서 사용 가능한 부동소수점 타입 : f32, f64
*/

fn main() {
    let twenty = 20; // 러스트는 타입이 지정되어 있지 않은 경우 사용자를 대신하여 해당 타입을 추론한다.
    let twenty_one: i32 = 21; // 타입 애너테이션(i32)를 붙여 타입을 지정한다.
    let twenty_two = 22i32; // 타입 접미사를 이용해 타입을 지정한다.

    let addition = twenty + twenty_one + twenty_two; 
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000; // 밑줄은 단지 가독성을 높여 주는 용도로 컴파일러는 이를 무시한다.
    println!("{}", one_million.pow(2)); // 숫자 타입은 메서드를 가진다.

    let forty_two = [ // 배열은 모두 같은 타입이어야 하며 대괄호 [] 로 묶어 생성한다.
        42.0, // 명시 적인 타입 애너테이션이 없는 부동 소수점 리터럴은 상황에 따라 32비트 또는 64비트가 된다.
        42f64, // 부동 소수점 리터럴에도 타입 접미사가 붙을 수 있다.
        42.0_f64, // 부동 소수점 리터럴과 타입 접미사 사이에 추가적인 밑줄을 쓸 수 있다.
    ];

    println!("{:02}", forty_two[0]); // 배열 내의 요소는 0부터 시작하여 숫자로 인덱싱이 가능
}