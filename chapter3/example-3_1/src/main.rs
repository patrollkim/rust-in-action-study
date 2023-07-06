#![allow(unused_variables)]  // 컴파일러 경고 완화 코드

type File = String; // 타입 별칭 선언 방법

fn open(f: &mut File) -> bool {
    true // 해당 함수는 항상 성공함
}

fn close(f: &mut File) -> bool {
    true // 해당 함수는 항상 성공함
}

#[allow(dead_code)] // 사용하지 않는 함수에 대한 컴파일 경고를 완화
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! { //! 반환 타입 : 이 함수가 절대로 어떤 값도 반환하지 않는다고 컴파일러에 알려주는 역활
    unimplemented!() // 프로그램이 해당 지점에 오면 중단 되는 매크로
}

fn main() {
    let mut f1 = File::from("f1.txt"); //여기서 File은 String의 타입 별칭이므로 String의 모든 메서드를 상속함
    open(&mut f1);
    // read(f1, vec![]); // 현재는 별의미 없는 함수이므로 주석 처리
    close(&mut f1);
}