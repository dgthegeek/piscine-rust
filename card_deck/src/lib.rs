use std::fmt;

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
    fn get_random() -> u8 {
        let time = std::time::SystemTime::now();
        let since_epoch = time.duration_since(std::time::UNIX_EPOCH).unwrap();
        since_epoch.as_secs() as u8
    }

    pub fn random() -> Suit {
        let random_value = (Self::get_random() % 4) + 1;
        Suit::translate(random_value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let random_value = (Suit::get_random() % 13) + 1;
        Rank::translate(random_value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            n @ 2..=10 => Rank::Number(n),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value"),
        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card {{ suit: {:?}, rank: {:?} }}", self.suit, self.rank)
    }
}

pub fn winner_card(card: &Card) -> bool {
    matches!(card, Card { suit: Suit::Spade, rank: Rank::Ace })
}
