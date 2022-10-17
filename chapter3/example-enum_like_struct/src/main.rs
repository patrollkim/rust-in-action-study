enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts, // 열거형의 마지막에는 리팩토링을 쉽게 하기 위해 마지막에 쉼표를 붙인다.
}

enum Card {
    King(Suit), // 얼굴 카드는 무늬가 있다.
    Queen(Suit), // 얼굴 카드는 무늬가 있다.
    Jack(Suit), // 얼굴 카드는 무늬가 있다.
    Ace(Suit), // 얼굴 카드는 무늬가 있다.
    Pip(Suit, usize), //일반 패틑 무늬와 등급(번호)로 구성된다.
}

fn main() {

}