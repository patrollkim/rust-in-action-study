fn main() {
    let num1 = 10;
    let num2 = 2;

    let result = add_with_lifetimes(&num1, &num2);

    println!("{} + {} = {}", num1, num2, result);
}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    i + j
}
