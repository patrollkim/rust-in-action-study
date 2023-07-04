#[derive(Debug)] // 열거형을 출력할때 println! 매크로를 사용할 수 있도록 설정
enum Cereal { // 열거형(enum)은 사용할 수 았는 값의 종류가 정해져 있는 타입이다.
    Barley,
    Millet, 
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![]; // Cereal을 항목으로 하는 빈 벡터 grains를 정의
    grains.push(Cereal::Rye); // grains 벡터에 항목 하나를 추가
    drop(grains); // grains 벡터와 내부 항목을 모두 메모리에서 삭제
    println!("{:?}", grains); // 삭제된 메모리에 접근 (댕글링 포인터 이슈)
}
