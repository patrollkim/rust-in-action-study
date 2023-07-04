fn main() {
    let fruit = vec!["키위", "바나나", "포도"];

    let buffer_overflow = fruit[4]; // 잘못된 위치의 메모리 값을 할당하는 대신 프로그램을 비정상(panic) 종료함.
    assert_eq!(buffer_overflow, "수박");
}
