#[derive(Debug,PartialEq)]
pub enum FileState { // 전체 열거형 타입을 공개로 지정하면 안의 열거값들도 모두 공개된다.
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>, // 해당 크레이트를 임포트 했을때 data는 비공개로 남는다. 
    pub state: FileState, 
}

impl File {
    pub fn new(name: &str) -> File { // File 구조체가 공개되었더라도 안의 메서드는 공개 여부를 명시적으로 지정해 주어야 한다.
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let f7: File = File::new("7.txt");

    println!("{:?}", f7);
    
}