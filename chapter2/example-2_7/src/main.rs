/*
  - 트레이트를 가져올때 특정 타입문 지역 범위로 가져오는것이 가능함.
  - while, for, loop 반복문 키워드
  - while과 for의 경우 종료 조건을 필 수로 지정
  - for의 경우 배열, 리스트 같은 자료구조의 순회가 가능
  - loop는 무한히 반복되는 형태 
*/

use std::time::{Duration, Instant}; // time 트레이트에 Duration, Instant 타입만 지역 범위로 가져옴. 

fn main() {

    let mut count = 0;
    let time_limit = Duration::new(1, 0); // 1초를 나타내는 Duration을 생성
    let start = Instant::now(); // 시스템의 내장 시계로부터 시간값을 읽어 온다.

    while (Instant::now() - start) < time_limit { // Instant에서 Instant를 빼면 Duration이 반환 된다.
        count += 1;
    } 
    
    println!("{}", count);
}
