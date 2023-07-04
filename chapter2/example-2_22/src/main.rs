fn main() {
    let context_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = Vec::new(); // tags에 일치하는 행의 번호를 저장한다.
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new(); // ctx는 일치하는 항목마다 문맥 앞뒤 행들을 저장하는 벡터를 보관한다.
    
    for (i, line) in haystack.lines().enumerate() { // 모든 줄에 대해 반복하면서 일치가 일어날 때 그 행 번호를 기록한다.
        if line.contains(needle) {
            tags.push(i);
            // Vec::with_capacity(n)는 n개의 항목을 위한 공간을 예약한다. 명시적인 타입 시그너처는 필요하지 않다.
            // 14행에서 ctx의 정의를 토대로 유추할 수 있기 때문이다. 
            let v = Vec::with_capacity(2 * context_lines + 1);
            ctx.push(v);
        }   
    }
    
    if tags.is_empty() { // 일치하는 항목이 없다면 프로그램을 종료한다.
        return;   
    }

    // 모든 태그에 대해 매 행마다 해당 행이 일치하는 곳 근처인지 검사한다. 해당하는 경우라면 ctx안에 있는 Vec<T>에 그 행을 추가한다.
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines); // sarurating_sub()는 뺄셈을 할 때 정수가 0보다 작아지면 프로그램을
            let upper_bound = tag + context_lines;               // 강제 종료하는 대신 0을 반환한다.(CPU는 usize 값이 0보다 작아지는 것
                                                                        // 을 용납 하지 않는다).
            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);  // 해당 행을 새로운 String으로 복사해서 일치할 때마다 지역 변수에 저장한다.
                let local_ctx = (i, line_as_string);

                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() { // ref line은 컴파일러에 이 값을 이동(move)하는 대신 대여(borrow)하려 한다고 알린다.
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}