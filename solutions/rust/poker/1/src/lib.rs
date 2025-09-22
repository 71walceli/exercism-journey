use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Suits {
    Club,
    Diamond,
    Heart,
    Spade,
}
#[derive(Debug)]
struct Card {
    rank: u8,
    suit: Suits,
}
impl TryFrom<&str> for Card {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.chars();
        let rank = match value.next().expect("Can't get rank from empty string") {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            '1' => match value.next().expect("Can't get next digit for rank") {
                '0' => 10,
                _ => return Err("Invalid rank value.")
            },
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => return Err("Can't match rank for given card string!")
        };

        let suit = match value.next().expect("Can't get rank, incomplete card string") {
            'C' => Suits::Club,
            'D' => Suits::Diamond,
            'H' => Suits::Heart,
            'S' => Suits::Spade,
            _ => return Err("invalid suite value")
        };
        Ok(Self { rank, suit,})
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}{}", 
            match self.rank {
                1 | 14 => "A",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                8 => "8",
                9 => "9",
                10 => "10",
                11 => "J",
                12 => "Q",
                13 => "K",
                _ => unreachable!() 
            }, 
            match self.suit {
                Suits::Club => "C",
                Suits::Diamond => "D",
                Suits::Heart => "H",
                Suits::Spade => "S",
            }
        );
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    RoyalFlush,
    StraightFlush(u8),
    FourOfAKind(u8, u8),
    FullHouse(u8, u8),
    Flush(u8, u8, u8, u8, u8),
    Straight(u8),
    ThreeOfAKind(u8, u8, u8),
    TwoPair(u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    HighCard(u8, u8, u8, u8, u8),
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use HandType::*;
        match (self, other) {
            (RoyalFlush, RoyalFlush) => std::cmp::Ordering::Equal,
            (RoyalFlush, _) => std::cmp::Ordering::Greater,
            (_, RoyalFlush) => std::cmp::Ordering::Less,

            (StraightFlush(r1), StraightFlush(r2)) => r1.cmp(r2),
            (StraightFlush(_), _) => std::cmp::Ordering::Greater,
            (_, StraightFlush(_)) => std::cmp::Ordering::Less,

            (FourOfAKind(r1, k1), FourOfAKind(r2, k2)) => r1.cmp(r2).then(k1.cmp(k2)),
            (FourOfAKind(_, _), _) => std::cmp::Ordering::Greater,
            (_, FourOfAKind(_, _)) => std::cmp::Ordering::Less,

            (FullHouse(t1, p1), FullHouse(t2, p2)) => t1.cmp(t2).then(p1.cmp(p2)),
            (FullHouse(_, _), _) => std::cmp::Ordering::Greater,
            (_, FullHouse(_, _)) => std::cmp::Ordering::Less,

            (Flush(r1, r2, r3, r4, r5), Flush(s1, s2, s3, s4, s5)) => r1.cmp(s1)
                .then(r2.cmp(s2))
                .then(r3.cmp(s3))
                .then(r4.cmp(s4))
                .then(r5.cmp(s5))
            ,

            (Flush(_, _, _, _, _), _) => std::cmp::Ordering::Greater,
            (_, Flush(_, _, _, _, _)) => std::cmp::Ordering::Less,

            (Straight(r1), Straight(r2)) => r1.cmp(r2),
            (Straight(_), _) => std::cmp::Ordering::Greater,
            (_, Straight(_)) => std::cmp::Ordering::Less,

            (ThreeOfAKind(r1, k1, k2), ThreeOfAKind(r2, k3, k4)) => r1.cmp(r2)
                .then(k1.cmp(k3))
                .then(k2.cmp(k4))
            ,

            (ThreeOfAKind(_, _, _), _) => std::cmp::Ordering::Greater,
            (_, ThreeOfAKind(_, _, _)) => std::cmp::Ordering::Less,
            (TwoPair(h1, l1, k1), TwoPair(h2, l2, k2)) => h1.cmp(h2)
                .then(l1.cmp(l2))
                .then(k1.cmp(k2))
            ,
            
            (TwoPair(_, _, _), _) => std::cmp::Ordering::Greater,
            (_, TwoPair(_, _, _)) => std::cmp::Ordering::Less,
            (OnePair(p1, k1, k2, k3), OnePair(p2, k4, k5, k6)) => p1.cmp(p2)
                .then(k1.cmp(k4))
                .then(k2.cmp(k5))
                .then(k3.cmp(k6))
            ,
            (OnePair(_, _, _, _), _) => std::cmp::Ordering::Greater,
            (_, OnePair(_, _, _, _)) => std::cmp::Ordering::Less,
            (HighCard(r1, r2, r3, r4, r5), HighCard(s1, s2, s3, s4, s5)) => r1.cmp(s1)
                .then(r2.cmp(s2))
                .then(r3.cmp(s3))
                .then(r4.cmp(s4))
                .then(r5.cmp(s5))
            ,
        }
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    r#type: HandType,
}
impl TryFrom<&str> for Hand {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cards = Hand::try_parse_cards(value)?;
        let r#type = Hand::score(&cards);

        if matches!(r#type, HandType::Straight(_, ) | HandType::StraightFlush(_, )) {
            let cards = cards.iter().map(|c| {
                match c.rank {
                    14 => Card { rank: 1, suit: c.suit },
                    _ => Card { rank: c.rank, suit: c.suit },
                }
            }).collect::<Vec<_>>();
        }

        // TODO Scoring
        Ok(Self {
            cards,
            r#type,
        })
    }
}
impl Hand {
    fn try_parse_cards(value: &str) -> Result<Vec<Card>, <Hand as TryFrom<&str>>::Error> {
        let cards = value.split_whitespace().enumerate();
        let mut new_hand = Vec::with_capacity(5);
        for (i, card) in cards {
            if i >= 5 {
                return Err("Too many cards")
            }
            new_hand.push(Card::try_from(card)?)
        }
        if new_hand.len() < 5 {
            return Err("Too few cards")
        }
        Ok(new_hand)
    }
    fn score(cards: &[Card]) -> HandType {
        let mut sorted_ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
        sorted_ranks.sort();
        sorted_ranks.reverse();

        let freqs = sorted_ranks.iter().fold(HashMap::new(), |mut acc, x| {
            let count = acc.get(x).unwrap_or(&0) + 1;
            acc.insert(*x, count);
            acc
        });

        let in_sequence = sorted_ranks.windows(2).all(|w| w[0] == w[1] + 1)
            || sorted_ranks == [14, 5, 4, 3, 2];

        if in_sequence && sorted_ranks == [14, 5, 4, 3, 2] {
            sorted_ranks = vec![5, 4, 3, 2, 1];
        }

        let all_same_suit = cards.iter().all(|c| c.suit == cards[0].suit);

        match (in_sequence, all_same_suit, &freqs) {
            (true, true, _) if sorted_ranks[0] == 14 => HandType::RoyalFlush,
            (true, true, _) => HandType::StraightFlush(sorted_ranks[0]),
            (_, true, _) => HandType::Flush(
                sorted_ranks[0],
                sorted_ranks[1],
                sorted_ranks[2],
                sorted_ranks[3],
                sorted_ranks[4],
            ),
            (true, false, _) => HandType::Straight(sorted_ranks[0]),
            (_, false, freqs) if freqs.len() == 2 => {
                let mut counts = freqs.iter().collect::<Vec<_>>();
                counts.sort_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)));
                if *counts[0].1 == 4 {
                    HandType::FourOfAKind(*counts[0].0, *counts[1].0)
                } else {
                    HandType::FullHouse(*counts[0].0, *counts[1].0)
                }
            }
            (_, false, freqs) if freqs.len() == 3 => {
                let mut counts = freqs.iter().collect::<Vec<_>>();
                counts.sort_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)));
                if *counts[0].1 == 3 {
                    HandType::ThreeOfAKind(*counts[0].0, *counts[1].0, *counts[2].0)
                } else {
                    HandType::TwoPair(*counts[0].0, *counts[1].0, *counts[2].0)
                }
            }
            (_, false, freqs) if freqs.len() == 4 => {
                let mut counts = freqs.iter().collect::<Vec<_>>();
                counts.sort_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)));
                HandType::OnePair(*counts[0].0, *counts[1].0, *counts[2].0, *counts[3].0)
            }
            _ => HandType::HighCard(
                sorted_ranks[0],
                sorted_ranks[1],
                sorted_ranks[2],
                sorted_ranks[3],
                sorted_ranks[4],
            ),
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", self.cards.iter().map(Card::to_string).collect::<Vec<String>>().join(" "));
        Ok(())
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.is_empty() {
        return vec![];
    }
    if hands.len() == 1 {
        return vec![hands[0]];
    }

    let hands = hands.iter().filter_map(|hand_str| {
        let hand = Hand::try_from(*hand_str).ok();
        if hand.is_none() {
            None
        } else {
            Some((hand.unwrap(), hand_str))
        }
    }).collect::<Vec<_>>();
    
    #[cfg(debug_assertions)] {
        hands.iter().for_each(|hand| println!("{:#?}", hand));
    }

    let max_card = hands.iter().max_by(|a, b| a.0.r#type.cmp(&b.0.r#type));
    let best_hands = hands.iter()
        .filter(|h| Some(&h.0.r#type) == max_card.map(|m| &m.0.r#type))
        .collect::<Vec<_>>()
    ;

    best_hands.iter().map(|(_, s)| **s).collect()
}
