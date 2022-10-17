#![allow(unused_variables)]

static mut ERROR: i32 = 0; // 정적(가변) 수명을 가져 프로그램의 수명(동작하는동안) 동안 유효하다.

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        read_length
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    let mut f = File::new("something.txt");
    let mut buffer: Vec<u8> = vec![]; 
    
    f.read(&mut buffer);
    unsafe { // 정적 가변 젼수에 접근해 이들 수정 하려면 unsafe 블록을 사용해야 한다.
        if ERROR != 0 { // ERROR 값을 검사한다. 오류 검사는 0은 오류가 아니라는 관례를 따른다.
            panic!("An error has occurred while reading the file ")
        }
    }

    close(&mut f);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while closing thr file ")
        }
    }

}