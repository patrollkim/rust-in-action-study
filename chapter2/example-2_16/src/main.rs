fn main() {
    println!("{} + {} = {}", 10, 12, add(10, 12));
}

fn add<T: std::ops::Add<Output=T>>(i: T, j: T) -> T {
    i + j
}
