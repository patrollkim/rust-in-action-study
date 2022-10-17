#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File { // 일반 함수로써 이 함수가 반환값이 File임을 rust에 알려준다.
        File {
            name: String::from(name), // 일반적인 객체 생성 구문을 캡슐화 한다.
            data: Vec::new(),
        }
    }
}

fn main() {
    let f3 = File::new("f3.txt");

    let f3_name = &f3.name; // 기본적으로 필드는 비공개지만 구조체를 정의한 모듈에서는 접근이 가능하다.
    let f3_length = f3.data.len();

    println!("{:?}", f3);
    println!("{} is {} bytes long", f3_name, f3_length);
}