fn main() {
let value: f32 = mock_rand(13);

    println!("{value}");
}

fn mock_rand(n: u8) -> f32 {
    (n as f32) / 255.0
}
