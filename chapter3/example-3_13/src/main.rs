// 파일 상태 열거형 타입 구현
#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

// 파일 구조체 구현
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

// new(), read() 메서드 구현
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

// open() 함수 구현
fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

// close() 함수 구현
fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

// 메인 로직
fn main() {
    let mut f6: File = File::new("6.txt");

    let mut buffer: Vec<u8> = vec![];

    if f6.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f6 = open(f6).unwrap();
    let f6_length = f6.read(&mut buffer).unwrap();
    f6 = close(f6).unwrap();

    let text = String::from_utf8_lossy(&buffer); 

    println!("{:?}", f6); // Debug는 :? 구문에 의존한다.
    println!("{} is {} bytes long", &f6.name, f6_length);
    println!("{}", text);
}