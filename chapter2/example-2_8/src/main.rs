/*
    if, match 조건문 키워드
    if의 경우 다른 언어와 사용법 동일
    러스트에서는 if문의 조건식에 ()을 사용안하는 것을 가이드 함.
    러스트에서 if는 표현식이므로 변수에 바로 바인딩 가능함.
    match 구문 역시 동일하게 변수에 바인딩 가능함.
    match 구문의 경우 조건에 대한 모든 경우의 수의 반환 로직이 구현되어야 함.
*/

fn main() {
    let needle = 42; // 사용하지 않으므로 불필요한 변수
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item { // match 표현식은 변수에 바인드될 값을 반환
            42 | 132 => "hit!", // | 는 or 연산자
            _ => "miss", // _는 모든 경우의 수와 일치하는 와일드 카드 패턴이다.
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}
