fn main() {
    let num1 = 10;
    let num2 = 2;

    let result = add(num1, num2);

    println!("{} + {} = {}", num1, num2, result);

}

fn add(i: i32, j: i32) -> i32 {
    i + j
}
