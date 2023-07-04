use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() { // BufReader::lines()는 각 줄에서 맨 뒤 개행 문자를 제거한다.
        let line = line_.unwrap(); // Result를 푼다. 하지만 오류 발생 시 프로그램이 강제 종료되는 위험을 감수한다.
        println!("{} ({} bytes long)", line, line.len());
    }
}