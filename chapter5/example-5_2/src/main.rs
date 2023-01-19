fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe {
        std::mem::transmute(a) // f32 타입 값을 u32 타입으로 변환
    };

    println!("{}", frankentype);
    println!("{:032b}", frankentype); // 값을 바이너리로 출력

    let b: f32 = unsafe {
        std::mem::transmute(frankentype) // u32 타입 값을 f32 타입으로 변환
    };

    println!("{}", b);
    assert_eq!(a, b);
}
