#[derive(Debug)] // File 구조체를 println!으로 출력 할 수 있게 해준다. Debug -> 트레이트로 {:?}완 연계하여 출력가능한 문자로 만들어 준다.
struct File {
    name: String,
    data: Vec<u8>, // 파일에 쓰기 작업을 시뮬레이트 하기위해서 Vector 자료형을 사용
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"), //String::from은 슬라이스인 문자열 리터럴에서 소유한 문자열을 생성
        data: Vec::new(), // 빈 파일을 시뮬레이트 한다.
    };

    let f1_name = &f1.name; // 구조체 필드에 접근 하려면 . 연산자를 사용한다. 
    let f1_length = &f1.data.len(); // &(참조)로 필드에 접근하면 소유권의 이동을 피할 수 있다.
                                            // Rust에서는 참조를 "대여한다." 라고 표현 하기도 한다.

    println!("{:?}", f1);
    println!("{}  is {} bytes long", f1_name, f1_length);
}