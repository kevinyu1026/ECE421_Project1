use deck::Deck;

mod deck;


fn main() {
    let mut deck = Deck::new(false);
    deck.shuffle();
    println!("{:?}", deck.card());
}
