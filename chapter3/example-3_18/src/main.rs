//! 한번에 한 단계씩 시뮬레이트 한다.

#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

// 백 틱으로 감싸면 문법 강조 기능도 지원이 된다.

impl File {
    /// 빈 `File `을 새로 만든다.
    /// 
    /// # 예제
    /// 
    /// ```
    /// let f  = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

fn main() {
    let f9 = File::new("9.txt");

    println!("{:?}", f9);
}