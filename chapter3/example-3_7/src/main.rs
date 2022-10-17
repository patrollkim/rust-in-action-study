use rand::{random}; // rand 크레이트 가져오기

static mut ERROR: isize = 0; // 전역 변수 ERROR를 0으로 초기화(정상상태)

struct File; // 실험할 구조체로 크기가 0인 타입을 만든다.

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() { // 여덟번중 한번은 참을 반환함
        unsafe {
            ERROR = 1; // 1로 초기화 하여 시스템 오류를 알림(오류상태)
        }
    }
    0 // 항상 0바이트를 읽은 것으로 간주한다.
}

#[allow(unused_mut)] // 실제로 값을 바꾸지는 않지만 코드의 일관성을 위해 버퍼를 가변 상태로 유지한다.
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe { // 정적 가변 변수 접근은 안전하지 않는 작업이다.
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}