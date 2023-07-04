fn main() {
    let num1 = 10;
    let num2 = 2;

    let result = add_with_lifetimes(&num1, &num2);

    let result = add_with_lifetime_hide(&num1, &num2);

    println!("{} + {} = {}", num1, num2, result);
}

/*
    - <'a, 'b>는 두 개의 수명 변수 'a와 'b를 add_with_lifetimes() 범위 안에서 선언한다.
    - 일반적으로 수명 a, 수명 b 라고 부른다.
    - i: &'a i32는 수명 변수 'a룰 i의 수명으로 바인드한다. 이 구문은 "매개 변수 i는 수명 a를 가지는 i32 타입의 참조다"를 말한다.
    - j: &'b i32는 수명 변수 'b를 j의 수명으로 바인드한다. 이 구문은 "매개 변수 j는 수명 b를 가지는 i32 타입의 참조다"를 말한다.
*/

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    i + j
}

fn add_with_lifetime_hide(i: &i32, j: &i32) -> i32 {
    i + j
}
