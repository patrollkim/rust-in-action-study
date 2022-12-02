#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

type Message = String;

fn main() {

    let sat_a = CubeSat { id: 100, mailbox: Mailbox { messages: vec![] } };
    
}
