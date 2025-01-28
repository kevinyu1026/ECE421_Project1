mod deck;

fn main() {
    // Create a new deck
    let mut deck = deck::Deck::new();

    // Shuffle the deck
    deck.shuffle();
    println!("Deck shuffled!");

    // Display all remaining cards in the deck
    println!("Remaining cards in the deck:");
    deck.display_remaining_cards();

    // Deal one card
    if let Some(card) = deck.deal_one_card() {
        println!("Dealt card: {}{}", card.rank, card.suit);
    } else {
        println!("No cards left to deal!");
    }

    // Display remaining cards after dealing
    println!("Remaining cards after dealing one card:");
    deck.display_remaining_cards();

    // Reset the deck
    deck.reset();
    println!("Deck reset!");

    // Display the deck after resetting
    println!("Deck after reset:");
    deck.display_remaining_cards();
}
