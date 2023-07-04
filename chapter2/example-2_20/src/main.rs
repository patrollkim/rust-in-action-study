fn main() {
    let search_term = "picture";
    
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with the books.
What do we seek through millions of pages?";

    for (i, line) in quote.lines().enumerate() { // lines()가 반복자를 반환하므로 이를 enumerate()와 결합할 수 있다.
        if line.contains(search_term) {
            let line_num = i + 1; // 행 번호를 덧셈을 통해 바로 계산하므로 반복 때마다 행 번호를 계산하던 일은 하지 않는다.
            println!("{}: {}", line_num, line);
        }
    }
}
