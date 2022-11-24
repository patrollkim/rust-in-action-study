/*
    - 알아볼것!
    1. &'static -> 정적 수명
    2. &[String]?
    3. Result 타입
    4. 예외처리 메서드 (unwrap, unwrap_or_else)
    5. 익명함수
    6. 클로저 (|test|)
    7. Box<dyn Error> 타입
    8. 테스트 코드 짜기 기법
    9. <'a> -> 명시적 수명
    10. 반복자 iter()
*/

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
 
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });

    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    };
}






