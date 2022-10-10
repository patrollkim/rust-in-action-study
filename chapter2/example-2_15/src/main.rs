fn main() {
    let num1 = 1;
    let num2 = 2;

    println!("{} + {} = {}", num1, num2, add(num1, num2));
}

fn add<T>(i: T, j: T) -> T {
    i + j // 연산 불가
}
