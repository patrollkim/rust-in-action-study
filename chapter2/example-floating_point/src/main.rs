fn main() {
    //assert!(0.1 + 0.2 == 0.3); // 참이 아니기 때문에 프로그램 강제 종료됨.

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!(" 0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!(" 0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!(" 0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!(" 0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    //assert!(xyz.0 + xyz.1 == xyz.2);
    
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);

    let x = (-42.0_f32).sqrt();
    //assert_eq!(x, x);

    let x: f32 = 1.0 / 0.0;
    assert!(x.is_finite()); 
}
