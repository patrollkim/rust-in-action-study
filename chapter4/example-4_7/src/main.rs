#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat { id }
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);

    sat_id
}

fn main() {
    let sat_a = CubeSat::new(0);
    let sat_b = CubeSat::new(1);
    let sat_c = CubeSat::new(2);

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
}
