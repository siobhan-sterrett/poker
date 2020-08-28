use crate::card::{PlayingCard, BaseHand};

pub trait HandResolutionPolicy {
    type Card;
    type Hand;

    fn hand(cards: [Self::Card; 5]) -> Self::Hand;
}

struct BaseHandResolutionPolicy;

impl HandResolutionPolicy for BaseHandResolutionPolicy {
    type Card = PlayingCard;
    type Hand = BaseHand;

    fn hand(cards: [PlayingCard; 5]) -> BaseHand {
        // TODO: Generify for wild cards

        // Check for straightness and flushness
        let flush = (1..5)
            .map(|i| cards[i])
            .all(|card| cards[0].suit == card.suit);
        let straight = (1..=13)
            .map(|i| (i..i+5)
                 .map(|j| ((j - 1) % 13) + 1))
            .any(|mut it| it.all(|i| cards.iter().any(|card| card.rank as i32 == i)));

        let mut counts = [0; 14];

        for card in &cards {
            counts[card.rank as usize] += 1;
        }

        let mut first_group = 0;
        let mut second_group = 0;

        for count in &counts {
            if count > &first_group {
                first_group = *count;
            } else if count > &second_group {
                second_group = *count;
            }
        }

        match (flush, straight, first_group, second_group) {
            (true, true, _, _) => BaseHand::StraightFlush,
            (true, false, _, _) => BaseHand::Straight,
            (false, true, _, _) => BaseHand::Flush,
            (false, false, 5, _) => BaseHand::FiveKind,
            (false, false, 4, _) => BaseHand::FourKind,
            (false, false, 3, 2) => BaseHand::FullHouse,
            (false, false, 3, _) => BaseHand::ThreeKind,
            (false, false, 2, 2) => BaseHand::TwoPair,
            (false, false, 2, _) => BaseHand::Pair,
            (false, false, _, _) => BaseHand::HighCard
        }
    }
}
