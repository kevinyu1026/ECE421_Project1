use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: char, // 'H', 'D', 'C', 'S'
    pub rank: String, // "2" to "10", "J", "Q", "K", "A"
}

pub struct Deck {
    cards: Vec<Card>,
    original_deck: Vec<Card>, // Store the original 52-card deck for resetting
}

impl Deck {
    // Create a new 52-card deck
    pub fn new() -> Self {
        let suits = ['H', 'D', 'C', 'S']; // Hearts, Diamonds, Clubs, Spades
        let ranks = vec![
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ];

        let mut cards = Vec::new();
        for &suit in &suits {
            for rank in &ranks {
                cards.push(Card {
                    suit,
                    rank: rank.to_string(),
                });
            }
        }

        // Store the original deck for resetting
        let original_deck = cards.clone();

        Self { cards, original_deck }
    }

    // Shuffle the deck
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    // Deal one card from the top of the deck
    pub fn deal_one_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    // Reset the deck back to the original 52 cards
    pub fn reset(&mut self) {
        self.cards = self.original_deck.clone();
        self.shuffle();
    }

    // Display the remaining cards in the deck
    pub fn display_remaining_cards(&self) {
        for card in &self.cards {
            println!("{}{}", card.rank, card.suit);
        }
    }
}

fn main() {
    // Create a new deck
    let mut deck = Deck::new();
    
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
