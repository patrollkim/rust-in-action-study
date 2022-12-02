#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

struct GroundStaion;

fn main() {
    let sat_a = CubeSat { id: 100, mailbox: Mailbox { messages: vec![] } };
}
