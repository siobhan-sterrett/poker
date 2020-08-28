#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Spades,
    Hearts
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    Ace = 1,
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
    King
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayingCard {
    pub rank: Rank,
    pub suit: Suit
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseHand {
    FiveKind, // Only possible with wild cards
    StraightFlush,
    FourKind,
    FullHouse,
    Flush,
    Straight,
    ThreeKind,
    TwoPair,
    Pair,
    HighCard
}
