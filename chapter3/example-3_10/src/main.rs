#[derive(Debug)] // 열거형을 화면에 출력
enum Event {
    Update, // 인식 할 수 없는 이벤트에 대한 값을 포함하여 세가지 Event 열거 값을 생성한다.
    Delete,
    Unknown,
}

type Message = String; // 이 크레이트 문맥에서 사용될 String의 편리한 이름이다.

fn parse_log(line: &'static str) -> (Event, Message) { // 행을 분석해 반구조화된 데이터로 변환하는 함수
    let parts: Vec<&str> = line.splitn(2, ' ').collect(); // collect는 line.splitn()에서 생성된 반복자를 써서 Vec<T>를 반환.

    if parts.len() == 1 { // line.splitn()이 로그를 두 부분으로 나누지 못한다면 오류를 반환 
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0]; // parts의 각 부분을 변수에 할당. (재사용을 편하게 하기 위해서)
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "Update" => (Event::Update, rest), // 알려진 이벤트의 경우 구조화 된 데이터를 반환
        "DELETE" | "Delete" => (Event::Delete, rest), // 알려진 이벤트의 경우 구조화 된 데이터를 반환
        _ => (Event::Unknown, String::from(line)), // 이벤트 타입을 모르면 전체 행을 반환.
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\":31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
