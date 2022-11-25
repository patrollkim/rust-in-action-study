#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
 
    let sat_a = CubeSat { id: 0 }; // 소유권이 발생
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let a_status = check_status(sat_a); // sat_a 객체의 소유권이 check_status() 로 이동하지만 main()으로 돌아오지 않는다.
    let b_status = check_status(sat_b); 
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    let a_status = check_status(sat_a); // sat_a는 더 이상 해당 객체의 소유자가 아니어서 해당 접근은 무효화 된다.
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

}
