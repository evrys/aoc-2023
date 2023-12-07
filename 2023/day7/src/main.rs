#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use itertools::Itertools;

use std::{cmp::Ordering, fs};

const _TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

const CARD_STRENGTH_PART_ONE: &str = "AKQJT98765432";
const CARD_STRENGTH_PART_TWO: &str = "AKQT98765432J";

struct Card {
    suit: char,
}

impl Clone for Card {
    fn clone(&self) -> Self {
        return Card { suit: self.suit };
    }
}

impl Card {
    fn strength_part1(&self) -> u32 {
        return (CARD_STRENGTH_PART_ONE.len() - CARD_STRENGTH_PART_ONE.find(self.suit).unwrap())
            as u32;
    }

    fn strength_part2(&self) -> u32 {
        return (CARD_STRENGTH_PART_TWO.len() - CARD_STRENGTH_PART_TWO.find(self.suit).unwrap())
            as u32;
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Clone for Hand {
    fn clone(&self) -> Self {
        return Hand {
            cards: self.cards.clone(),
            bid: self.bid,
        };
    }
}

impl Hand {
    fn handtype_part1(&self) -> u32 {
        let uniq_cards = self.cards.iter().unique_by(|c| c.suit).collect::<Vec<_>>();
        let mut counts = uniq_cards
            .iter()
            .map(|&c| self.cards.iter().filter(|&d| d.suit == c.suit).count())
            .collect::<Vec<_>>();
        counts.sort();

        if counts == [5] {
            // Five of a kind
            return 7;
        } else if counts == [1, 4] {
            // Four of a kind
            return 6;
        } else if counts == [2, 3] {
            // Full house
            return 5;
        } else if counts == [1, 1, 3] {
            // Three of a kind
            return 4;
        } else if counts == [1, 2, 2] {
            // Two pair
            return 3;
        } else if counts == [1, 1, 1, 2] {
            // One pair
            return 2;
        } else {
            // High card
            return 1;
        }
    }

    fn handtype_part2(&self) -> u32 {
        let jokers = self.cards.iter().filter(|&c| c.suit == 'J');
        let num_jokers = jokers.count();

        if num_jokers == 5 {
            // Five of a kind
            return 7;
        }

        let non_jokers = self.cards.iter().filter(|&c| c.suit != 'J');
        let uniq_non_jokers = non_jokers.unique_by(|c| c.suit);
        let mut counts = uniq_non_jokers
            .map(|c| self.cards.iter().filter(|&d| d.suit == c.suit).count())
            .collect::<Vec<_>>();
        let total = counts.len();
        counts.sort();

        // Jokers become the most relevant card they can
        counts[total - 1] += num_jokers;

        if counts == [5] {
            // Five of a kind
            return 7;
        } else if counts == [1, 4] {
            // Four of a kind
            return 6;
        } else if counts == [2, 3] {
            // Full house
            return 5;
        } else if counts == [1, 1, 3] {
            // Three of a kind
            return 4;
        } else if counts == [1, 2, 2] {
            // Two pair
            return 3;
        } else if counts == [1, 1, 1, 2] {
            // One pair
            return 2;
        } else {
            // High card
            return 1;
        }
    }
}

fn compare_hand_strength_part1(hand1: &Hand, hand2: &Hand) -> Ordering {
    let handtype1 = hand1.handtype_part1();
    let handtype2 = hand2.handtype_part1();

    if handtype1 > handtype2 {
        return Ordering::Greater;
    } else if handtype1 < handtype2 {
        return Ordering::Less;
    } else {
        for i in 0..hand1.cards.len() {
            let s1 = hand1.cards[i].strength_part1();
            let s2 = hand2.cards[i].strength_part1();

            if s1 > s2 {
                return Ordering::Greater;
            } else if s1 < s2 {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let mut hands = puzzle_input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let cards = iter
                .next()
                .unwrap()
                .chars()
                .map(|suit| Card { suit })
                .collect::<Vec<_>>();
            let bid = iter.next().unwrap().parse::<u32>().unwrap();

            Hand { cards, bid }
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| compare_hand_strength_part1(a, b));

    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = i + 1;
        winnings += hand.bid * rank as u32;
    }

    println!("Part 1: {}", winnings);
}

fn compare_hand_strength_part2(hand1: &Hand, hand2: &Hand) -> Ordering {
    let handtype1 = hand1.handtype_part2();
    let handtype2 = hand2.handtype_part2();

    if handtype1 > handtype2 {
        return Ordering::Greater;
    } else if handtype1 < handtype2 {
        return Ordering::Less;
    } else {
        for i in 0..hand1.cards.len() {
            let s1 = hand1.cards[i].strength_part2();
            let s2 = hand2.cards[i].strength_part2();

            if s1 > s2 {
                return Ordering::Greater;
            } else if s1 < s2 {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let mut hands = puzzle_input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let cards = iter
                .next()
                .unwrap()
                .chars()
                .map(|suit| Card { suit })
                .collect::<Vec<_>>();
            let bid = iter.next().unwrap().parse::<u32>().unwrap();

            Hand { cards, bid }
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| compare_hand_strength_part2(a, b));

    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = i + 1;
        winnings += hand.bid * rank as u32;
    }

    println!("Part 2: {}", winnings);
}

fn main() {
    part1();
    part2();
}
