#![allow(dead_code)] // FileState::Open 이 쓰이지 않는다는 경고를 제거

use std::fmt; // fmt::Result, fmt::Formatter를 사용할 수 있도록 std:fmt 크레이트를 가져온다.
use std::fmt::{Display}; // Display 크레이트를 가져온다.

#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),  // String은 이미 Display가 구현되어있다. (덕분에 아무조치 없이 출력가능)
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} => {}>", self.name, self.state) // self.state는 FileState의 Display 구현에 의존한다.
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }       
    }
}

fn main() {
    let f7 = File::new("7.txt");

    println!("{:?}", f7);
    println!("{}", f7); // File의 Display 구현은 자신의 규칙에 따라 출력한다. line 30
}