fn main() {
    let mut letters = vec!["a", "b", "b"]; // 가변 벡터 생성

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone()); // 각 글자를 복제한 후 벡터 끝에 덧 붙인다.
        // letters 벡터를 반복문 안에서 수정 되는 것을 허용하지 않음.
    }

}
