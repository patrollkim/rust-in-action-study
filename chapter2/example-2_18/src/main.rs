fn main() {
    let search_term = "picture";
    let quote ="\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with the books.
What do we seek through millions of pages?"; // 여러 줄로 된 문자열을 위한 특별한 문법은 없다. 3행의 \이 줄 바꿈을 방지한다.

    for line in quote.lines() {   // lines()는 auote에 대해 매 반복마다 텍스트를 한 줄씩 돌려주는 반복자를 반환한다.
        if line.contains(search_term) { // 러스트는 줄 바꿈을 할 때 각 운영체제의 관례를 따른다.
            println!("{}", line);
        } 
    }
}
