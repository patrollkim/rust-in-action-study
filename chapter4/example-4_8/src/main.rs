/*
- 소유권을 다루는 전략
    1. 완전한 소유권이 필요하지 않은 경우에는 참조를 사용한다.
    2. 값을 복제한다.
    3. 장기간 유지되어야 하는 객체 수를 줄일 수 있도록 코드를 리팩터링한다.
    4. 이동 문제를 보조하기 위해 설계된 타입으로 데이터를 감싼다.

*/

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    message: Vec<Message>,
}

type Message = String;

fn main() {
    
}
