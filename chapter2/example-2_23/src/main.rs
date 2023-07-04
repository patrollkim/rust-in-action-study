fn main() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, publlic-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books, What do we seek through millions of pages?";

    for line in quote.lines() {
        if line.contains(search_term) { // contains() 매서드는 부분 문자열을 검색한다.
            println!("{}", line);
        }
    }
}
