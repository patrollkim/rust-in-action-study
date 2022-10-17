#![allow(unused_variables)] // 경고 문고 표시 제거

#[derive(Debug)] // File 구조체를 문자열로 출력 가능하도록 설정
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool { // 비활성상태 (아직 사용안됨)
    true
}

fn close(f: &mut File) -> bool { // 비활성상태 (아직 사용안됨)
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize { // 읽은 바이트수를 반환
    let mut tmp = f.data.clone(); // save_to.append() 메서드 사용시 입력값 Vec<T> 가 줄어 드므로 미리 복사
    let read_length = tmp.len(); 

    save_to.reserve(read_length); // 데이터 저장 공간 충분한지 체크
    save_to.append(&mut tmp); // save_to 버퍼에 충분한 공간 할당(f(FILE)의 내용을 담음)
    
    read_length
}

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);                                            // 파일과 상호작용함.
    let f2_length = read(&f2, &mut buffer); // 파일과 상호작용함.
    close(&mut f2);                                           // 파일과 사용작용함.

    let text = String::from_utf8_lossy(&buffer); // Vec<u8> 을 String(utf8)으로 변환. 올바른 UTF-8이 아닌 경우 ?로 바뀜

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text); // f2.data의 바이트를 문자열로 표시
}