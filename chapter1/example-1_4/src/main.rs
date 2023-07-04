use std::thread; // thread 기능을 사용하도록 가져온다.

fn main() {
    let mut data = 100;

    thread::spawn(|| { data = 500; }); // thread::spawn() 메서드는 클로저(익명함수)를 인자로 받는다.
    thread::spawn(|| { data = 1000; });
    println!("{}", data); // 변수에 대한 멀티 스레드의 다중 접근으로 문제 발생
}
