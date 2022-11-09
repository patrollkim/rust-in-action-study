
fn main() {
    let three = 0b11; // 0b 접두사는 2진수임을 나타냄.
    let thirty = 0o36; // 0o 접두사는 8진수임을 나타냄.
    let three_hundred = 0x12C; // 0x 접두사는 16진수임을 나타냄. 

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base  2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base  8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
