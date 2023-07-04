// Rust 코드를 실행하기 위해서는 main() 함수가 꼭 필요하다.

fn main() {
    let penguin_data = "\
    common name, length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 { // 헤더 및 공백이 있는 줄은 처리하지 않음.
            continue;
        }

        let fields: Vec<_> = record // 한줄 텍스트로 시작
            .split(",") // record와 field로 ,를 구분하여 나눔
            .map(|field| field.trim()) // 각 filed의 양쪽 공백을 삭제
            .collect(); // field 모음을 벡터 하나로 합친다.
        
        if cfg!(debug_assertions) { // cfg! 매크로는 컴파일 환경 설정을 검사한다. 이 경우 debug 모드인지를 확인한다.
            eprintln!("debug: {:?} -> {:?}", record, fields); // 표준오류(stderr)로 내용을 출력한다.
        }

        let name = fields[0]; // field의 이름 텍스트를 가져온다.
        if let Ok(length) = fields[1].parse::<f32>() { // 부동 소수점 수로 분석한다. 
            println!("{}, {}cm", name, length); // 표준 출력(stdout)으로 출력한다. 
        }
    }
}
