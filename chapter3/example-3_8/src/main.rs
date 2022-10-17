use rand::prelude::*; // rand 크레이트에서 공통 크레이트와 타입을 현재 크레이트 범위로 가져온다.

fn one_in(denominator: u32) -> bool { // 산발적 오류를 일으키는 도움 함수
    thread_rng().gen_ratio(1, denominator) // thread_rng() 함수는 스레드 로컬 난수 생성기를 만든다. gen_ratio(n, m)은 n/m 확룔을 가지는 bool 값을 반환한다.
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> { // Result<T, E> 타입, 여기에서 T는 부호없는 정수 타입 usize 이며 E는 String 이다.
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length) // 여기서 read는 절대 실패하지 않으나 Result 타입을 반환해야 하므로 Ok로 감싼다.
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(5) { // 파라미터 숫자 번에 1번 꼴로 오류를 반환한다.
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(5) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    f4 = open(f4).unwrap(); // Ok 로 부터 T를 풀어 T를 남긴다.
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}