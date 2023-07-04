fn main() {
    let search_term = "picture";
    
    // 역슬래시로 문자열 리터럴의 개행 문자를 이스케이프한다.
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with the books.
What do we seek through millions of pages?";

    let mut line_num: usize = 1; // let mut로 line_num을 변경 가능하다고 선언하고 값을 1로 초기화한다.

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}: {}", line_num, line);  // println! 매크로에서 두 값 모두 출력할 수 있도록 수정한다.
        }
        line_num += 1; // line_num을 증가시킨다.
    }
}
