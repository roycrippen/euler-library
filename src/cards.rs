// #![feature(plugin)]
//
// #![plugin(clippy)]

use self::Val::*;
use self::Suit::*;

use std::fmt;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Val { Two = 2, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace, }

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit { Spades, Hearts, Diamonds, Clubs, }

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub val: Val,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({:?}, {:?})", self.val, self.suit) }
}

impl Card {
    pub fn same_suit(&self, other: Card) -> bool { self.suit == other.suit }
    pub fn next_val(&self) -> Val {
        match self.val {
            Ace => Two,
            Two => Three,
            Three => Four,
            Four => Five,
            Five => Six,
            Six => Seven,
            Seven => Eight,
            Eight => Nine,
            Nine => Ten,
            Ten => Jack,
            Jack => Queen,
            Queen => King,
            King => Ace,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn show(&self) -> String {
        let mut str = "[".to_string();
        for x in self.cards.iter() {
            str.push_str(&format!("{}, ", x))
        }
        str.pop();
        str.pop();
        str = str + &"]";
        str
    }
    pub fn is_flush(&self) -> bool {
        let first = self.cards[0].suit;
        self.cards.iter().skip(1).all(|x| x.suit == first)
    }
    pub fn is_straight(&self) -> bool {
        let mut card = self.cards.clone();

        // handle Ace low straight
        if card[0].val == Two && card[4].val == Ace {
            let t = card.pop().unwrap();
            card.reverse();
            card.push(t);
            card.reverse();
        }

        let mut first = card[0];
        for v in card.iter().skip(1) {
            if v.val != first.next_val() {
                return false;
            }
            first = *v;
        }
        true
    }
    pub fn is_straight_flush(&self) -> bool {
        if !self.is_flush() {
            return false;
        }
        self.is_straight()
    }
    pub fn is_3_of_kind(&self, gss: &[Hand]) -> bool {
        for gs in gss {
            if gs.cards.len() == 3 {
                return true;
            }
        }
        false
    }
    pub fn is_4_of_kind(&self, gss: &[Hand]) -> bool {
        for gs in gss {
            if gs.cards.len() == 4 {
                return true;
            }
        }
        false
    }
    pub fn is_pair(&self, gss: &[Hand]) -> bool {
        for gs in gss {
            if gs.cards.len() == 2 {
                return true;
            }
        }
        false
    }
    pub fn is_2_pair(&self, gss: &[Hand]) -> bool {
        let mut p1 = false;
        let mut p2 = false;

        for gs in gss {
            if gs.cards.len() == 2 {
                if p1 {
                    p2 = true;
                } else {
                    p1 = true
                }
            }
        }
        if !p1 || !p2 {
            return false;
        }
        true
    }
    pub fn is_full_house(&self, gss: &[Hand]) -> bool {
        if !self.is_pair(gss) {
            return false;
        }
        self.is_3_of_kind(gss)
    }
    pub fn is_high_card(&self, gss: &[Hand]) -> bool {
        if gss.len() == 5 {
            return true;
        }
        false
    }
    pub fn group(&self) -> Vec<Hand> {
        let mut sorted = self.cards.clone();
        sorted.sort();
        let mut res: Vec<Hand> = Vec::new();
        let hand: &mut Vec<Card> = &mut Vec::new();
        for v in sorted {
            if hand.is_empty() {
                hand.push(v);
                continue;
            }
            if v.val == hand.last().unwrap().val {
                hand.push(v);
                continue;
            } else {
                res.push(Hand { cards: hand.clone() });
                hand.clear();
                hand.push(v);
            }
        }
        if !hand.is_empty() {
            res.push(Hand { cards: hand.clone() });
        }
        res
    }
    pub fn get_rank(&self) -> usize {
        let mut cards = self.cards.clone();
        cards.sort();
        let hand = Hand { cards: cards };
        let group = hand.group();
        let mut rank: usize = 0;
        if hand.is_straight_flush() {
            rank = 8_000_000 + hand.value_high_card()
        } else if hand.is_4_of_kind(&group) {
            rank = 7_000_000 + (hand.cards[2].val as usize)
        } else if hand.is_full_house(&group) {
            rank = 6_000_000 + (hand.cards[2].val as usize)
        } else if hand.is_flush() {
            rank = 5_000_000 + hand.value_high_card()
        } else if hand.is_straight() {
            rank = 4_000_000 + hand.value_high_card()
        } else if hand.is_3_of_kind(&group) {
            rank = 3_000_000 + (hand.cards[2].val as usize)
        } else if hand.is_2_pair(&group) {
            rank = 2_000_000 + hand.value_2_pair(&group)
        } else if hand.is_pair(&group) {
            rank = 1_000_000 + hand.value_pair(&group)
        } else if hand.is_high_card(&group) {
            rank = hand.value_high_card()
        }
        rank
    }
    pub fn value_high_card(&self) -> usize {
        let mut mult = 1;
        let mut res = 0;
        for v in &self.cards {
            res += mult * v.val as usize;
            mult *= 14;
        }
        res
    }
    pub fn value_pair(&self, gss: &[Hand]) -> usize {
        let mut mult = 1;
        let mut res = 0;
        let mut pair = 0;
        for gs in gss {
            if gs.cards.len() == 2 {
                pair = gs.cards[0].val as usize;
                res += 14 * 14 * 14 * pair
            }
        }
        for v in &self.cards {
            if v.val as usize != pair {
                res += mult * v.val as usize;
                mult *= 14;
            }
        }
        res
    }
    pub fn value_2_pair(&self, gss: &[Hand]) -> usize {
        let mut res = 0;
        let mut pair1 = 0;
        let mut pair2 = 0;
        for gs in gss {
            if gs.cards.len() == 2 {
                if pair1 == 0 {
                    pair1 = gs.cards[0].val as usize;
                } else {
                    pair2 = gs.cards[0].val as usize;
                }
            } else {
                res += gs.cards[0].val as usize
            }
        }
        if pair1 > pair2 {
            res += 14 * 14 * pair1 + 114 * pair2;
        } else {
            res += 14 * 14 * pair2 + 114 * pair1;
        }
        res
    }
}

pub fn show_grp(gss: Vec<Hand>) -> String {
    let mut str = "".to_string();
    for gs in gss {
        str = str + &gs.show() + "\n";
    }
    str
}

pub fn char_to_suit(c: char) -> Suit {
    match c {
        'S' => Spades,
        'H' => Hearts,
        'D' => Diamonds,
        'C' => Clubs,
        _ => panic!(format!("error getting suit: {}", c)),
    }
}

pub fn char_to_val(c: char) -> Val {
    match c {
        '2' => Two,
        '3' => Three,
        '4' => Four,
        '5' => Five,
        '6' => Six,
        '7' => Seven,
        '8' => Eight,
        '9' => Nine,
        'T' => Ten,
        'J' => Jack,
        'Q' => Queen,
        'K' => King,
        'A' => Ace,
        _ => panic!(format!("error getting value: {}", c)),
    }
}
