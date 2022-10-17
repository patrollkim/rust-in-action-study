use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { // std::fmt::Display 를 구현하려면 fmt메서드가 해당 타입에 정의되어야 한다.
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} {}>", self.name, self.state) // write! 매크로를 통해 내부 타입의 Display 구현을 따르는것이 일반적이다. 
    }
}

fn main() {

}