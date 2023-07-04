use regex::Regex; // regex 크레이트의 Regex 타입을 지역 범위로 가져온다.

fn main() {
    let re = Regex::new("picture").unwrap(); // unwrap()은 Result값을 풀어내는데 오류가 발생하면 강제 종료(panic)한다.

    let quote = "Every face, every shop, bedroom window, public-house and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through miliions od pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);

        match contains_substring { // 이전에 사용한 contains() 매서드를 match 블록으로 대치하여 발생 가능한 모든 경우에 대처한다.
            Some(_) => println!("{}", line), // Some(T)는 Option 타입의 값 중 긍정적인 경우로, re.find가 성공했다는 의미다. 결과가 있는 모든 경우에 해당한다.
            None => (), // None은 Option 타입의 값 중 부정적인 경우다. ()는 Null 자리 표시자로 볼 수 있다.
        }
    }
}
