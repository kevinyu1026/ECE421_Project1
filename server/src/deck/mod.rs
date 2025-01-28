//! This module contains the implementation of the deck of cards.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

/// Represents the rank of a card.
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

/// Represents a card in the deck.
pub struct Card {
    /// The suit of the card.
    pub suit: Suit,
    /// The rank of the card.
    pub rank: Rank,
}

pub struct Deck {
    pub cards: Vec<Card>,
}


impl Deck{
    /// Creates a new deck of cards.
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in &[Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds] {
            for &rank in &[Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] {
                cards.push(Card { suit: suit.clone(), rank });
            }
        }
        Deck { cards }
    }
    /// Shuffles the deck of cards.
    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deals a card from the deck.
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

}
