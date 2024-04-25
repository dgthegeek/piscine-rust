#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Suit {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                let seconds = duration.as_secs();
                match seconds % 4 {
                    0 => Suit::Heart,
                    1 => Suit::Diamond,
                    2 => Suit::Spade,
                    _ => Suit::Club,
                }
            }
            Err(_) => Suit::Heart, // Fallback if system time retrieval fails
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                let seconds = duration.as_secs();
                match seconds % 13 {
                    0 => Rank::Ace,
                    10 => Rank::Jack,
                    11 => Rank::Queen,
                    12 => Rank::King,
                    n => Rank::Number(n as u8 + 1),
                }
            }
            Err(_) => Rank::Number(1), // Fallback if system time retrieval fails
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n),
        }
    }
}
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    match (&card.suit, &card.rank) {
        (Suit::Spade, Rank::Ace) => true,
        _ => false,
    }
}
