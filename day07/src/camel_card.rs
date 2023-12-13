#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CamelCardRank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq)]
pub struct CamelCardHand<'a> {
    rank: CamelCardRank,
    hand: &'a str,
}

impl<'a> From<&'a str> for CamelCardHand<'a> {
    fn from(value: &'a str) -> Self {
        let mut cards_in_hand = value.chars().collect::<Vec<_>>();
        cards_in_hand.sort_unstable();
        let rank = match cards_in_hand.as_slice() {
            [card1, card2, card3, card4, card5]
                if card1 == card2 && card1 == card3 && card1 == card4 && card1 == card5 =>
            {
                CamelCardRank::FiveOfAKind
            }
            [card1, card2, card3, card4, _]
                if card1 == card2 && card1 == card3 && card1 == card4 =>
            {
                CamelCardRank::FourOfAKind
            }
            [_, card2, card3, card4, card5]
                if card2 == card3 && card2 == card4 && card2 == card5 =>
            {
                CamelCardRank::FourOfAKind
            }
            [card1, card2, card3, card4, card5]
                if card1 == card2 && card3 == card4 && card3 == card5 =>
            {
                CamelCardRank::FullHouse
            }
            [card1, card2, card3, card4, card5]
                if card1 == card2 && card1 == card3 && card4 == card5 =>
            {
                CamelCardRank::FullHouse
            }
            [card1, card2, card3, _, _] if card1 == card2 && card1 == card3 => {
                CamelCardRank::ThreeOfAKind
            }
            [_, card2, card3, card4, _] if card2 == card3 && card2 == card4 => {
                CamelCardRank::ThreeOfAKind
            }
            [_, _, card3, card4, card5] if card3 == card4 && card3 == card5 => {
                CamelCardRank::ThreeOfAKind
            }
            [card1, card2, card3, card4, _] if card1 == card2 && card3 == card4 => {
                CamelCardRank::TwoPairs
            }
            [card1, card2, _, card4, card5] if card1 == card2 && card4 == card5 => {
                CamelCardRank::TwoPairs
            }
            [_, card2, card3, card4, card5] if card2 == card3 && card4 == card5 => {
                CamelCardRank::TwoPairs
            }
            [card1, card2, _, _, _] if card1 == card2 => CamelCardRank::OnePair,
            [_, card2, card3, _, _] if card2 == card3 => CamelCardRank::OnePair,
            [_, _, card3, card4, _] if card3 == card4 => CamelCardRank::OnePair,
            [_, _, _, card4, card5] if card4 == card5 => CamelCardRank::OnePair,
            _ => CamelCardRank::HighCard,
        };

        Self { rank, hand: value }
    }
}

impl<'a> PartialOrd for CamelCardHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for CamelCardHand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => self
                .hand
                .chars()
                .zip(other.hand.chars())
                .find_map(|(lhs, rhs)| {
                    if lhs == rhs {
                        None
                    } else {
                        match (lhs, rhs) {
                            ('A', _) => Some(std::cmp::Ordering::Greater),
                            (_, 'A') => Some(std::cmp::Ordering::Less),
                            ('K', _) => Some(std::cmp::Ordering::Greater),
                            (_, 'K') => Some(std::cmp::Ordering::Less),
                            ('Q', _) => Some(std::cmp::Ordering::Greater),
                            (_, 'Q') => Some(std::cmp::Ordering::Less),
                            ('J', _) => Some(std::cmp::Ordering::Greater),
                            (_, 'J') => Some(std::cmp::Ordering::Less),
                            ('T', _) => Some(std::cmp::Ordering::Greater),
                            (_, 'T') => Some(std::cmp::Ordering::Less),
                            (l, r) => Some(l.cmp(&r)),
                        }
                    }
                })
                .unwrap_or(std::cmp::Ordering::Equal),
            ord => ord,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct JokerCamelCardHand<'a> {
    rank: CamelCardRank,
    hand: &'a str,
}

impl<'a> From<&'a str> for JokerCamelCardHand<'a> {
    fn from(value: &'a str) -> Self {
        let mut cards = value.chars().filter(|c| c != &'J').collect::<Vec<_>>();
        cards.sort_unstable();
        let rank = match cards.as_slice() {
            [_, _, _, _, _] => {
                let camel: CamelCardHand = value.into();
                camel.rank
            }
            [a, b, c, d] if a == b && a == c && a == d => CamelCardRank::FiveOfAKind,
            [a, b, c, _] if a == b && a == c => CamelCardRank::FourOfAKind,
            [_, a, b, c] if a == b && a == c => CamelCardRank::FourOfAKind,
            [a, b, c, d] if a == b && c == d => CamelCardRank::FullHouse,
            [a, b, _, _] if a == b => CamelCardRank::ThreeOfAKind,
            [_, a, b, _] if a == b => CamelCardRank::ThreeOfAKind,
            [_, _, a, b] if a == b => CamelCardRank::ThreeOfAKind,
            [_, _, _, _] => CamelCardRank::OnePair,
            [a, b, c] if a == b && a == c => CamelCardRank::FiveOfAKind,
            [a, b, _] if a == b => CamelCardRank::FourOfAKind,
            [_, a, b] if a == b => CamelCardRank::FourOfAKind,
            [_, _, _] => CamelCardRank::ThreeOfAKind,
            [a, b] if a == b => CamelCardRank::FiveOfAKind,
            [_, _] => CamelCardRank::FourOfAKind,
            _ => CamelCardRank::FiveOfAKind,
        };

        Self { rank, hand: value }
    }
}

impl<'a> PartialOrd for JokerCamelCardHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for JokerCamelCardHand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => self
                .hand
                .chars()
                .zip(other.hand.chars())
                .find_map(|(lhs, rhs)| {
                    if lhs == rhs {
                        None
                    } else {
                        match (lhs, rhs) {
                            ('A', _) => Some(std::cmp::Ordering::Greater),
                            (_, 'A') => Some(std::cmp::Ordering::Less),
                            ('K', _) => Some(std::cmp::Ordering::Greater),
                            (_, 'K') => Some(std::cmp::Ordering::Less),
                            ('Q', _) => Some(std::cmp::Ordering::Greater),
                            (_, 'Q') | ('J', _) => Some(std::cmp::Ordering::Less),
                            (_, 'J') | ('T', _) => Some(std::cmp::Ordering::Greater),
                            (_, 'T') => Some(std::cmp::Ordering::Less),
                            (l, r) => Some(l.cmp(&r)),
                        }
                    }
                })
                .unwrap_or(std::cmp::Ordering::Equal),
            ord => ord,
        }
    }
}
