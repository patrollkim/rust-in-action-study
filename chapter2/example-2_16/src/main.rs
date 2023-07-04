fn main() {
    println!("{} + {} = {}", 10, 12, add(10, 12));
}

// Add 트레이트 제약을 포함하였기때문에 가능하다.
fn add<T: std::ops::Add<Output=T>>(i: T, j: T) -> T {
    i + j
}
