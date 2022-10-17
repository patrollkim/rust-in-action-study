#[derive(Debug)]
enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn main() {

    let mut suit: Vec<Suit> = vec![]; 

    suit.push(Suit::Clubs);
    suit.push(Suit::Spades);
    suit.push(Suit::Diamonds);
    suit.push(Suit::Hearts);
    

    println!("{:?}", suit);
}