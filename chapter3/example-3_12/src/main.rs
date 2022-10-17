#![allow(unused_variables)] // 함수 내에서 사용하지 않는 변수에 대한 경고를 내지 않도록 해준다.

#[derive(Debug)]
struct File; // 빈 File 타입을 선언

trait Read { // 트레이트에 특정 이름을 지정
    // 트레이트 블록은 구현제가 반드시 따라야 할 함수의 시그니처 타입을 포함한다. 
    // 의사(pseudo)타입 self는 Read를 구현하는 타입에 대한 자리 표시자이다.
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>; 
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0) // 필요한 타입 시그니처를 준수하는 단순 스터브 값
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec!();
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}