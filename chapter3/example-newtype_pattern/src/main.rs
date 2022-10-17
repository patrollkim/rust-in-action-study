struct Hostname(String); // 단일 필드 구조체(또는 튜플)로 핵심 타입을 감싸는 패턴

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());

    connect(host); // 타입이 매칭됨.
    connect(ordinary_string); // String 타입과 Hostname(String을 감싼)은 다른 타입으로 인식
}