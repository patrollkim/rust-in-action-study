fn main() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in &haystack { // haystack 요소의 참조에 대해 반복한다.
        if *item == needle { // *item은 item의 대상을 반환한다.
            println!("needle match : {}", item);
        } else {
            println!("_ : {}", item);
        }
    }
}
