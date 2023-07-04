fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b); // 수명 애너테이션은 함수 호출 시에는 필요하지 않다.

    println!("{}", res);
}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j // 참조를 직접 더하는 대신 i와 j를 역참조하여 그 값을 더한다.
}
