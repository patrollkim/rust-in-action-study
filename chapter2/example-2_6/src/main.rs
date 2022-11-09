use num::complex::Complex; // Complex 구조체 타입을 지역 범위로 가져옴

fn main() {
    let a = Complex { re: 2.1, im: -1.2 }; // 모든 러스트 타입은 리터럴 구문을 가지고 있다.
    let b = Complex::new(11.1, 22.2); // 대부분의 데이터 타입은 정적 메서드 new()를 구현함.  
    let result = a + b;
    
    println!("{} + {}i", result.re, result.im); // .연산자를 통해 데이터 필드에 접근 한다.
}

