use std::cmp::Ordering;
use indexmap::IndexMap;
use crate::read_lines::read_lines;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
enum PlayingCard {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Undefined = 0
}

impl PlayingCard {
    fn get_card_type_from_char(c: char) -> Result<PlayingCard, ()> {
        return match c {
            'A' => Ok(PlayingCard::Ace),
            'K' => Ok(PlayingCard::King),
            'Q' => Ok(PlayingCard::Queen),
            'J' => Ok(PlayingCard::Jack),
            'T' => Ok(PlayingCard::Ten),
            '9' => Ok(PlayingCard::Nine),
            '8' => Ok(PlayingCard::Eight),
            '7' => Ok(PlayingCard::Seven),
            '6' => Ok(PlayingCard::Six),
            '5' => Ok(PlayingCard::Five),
            '4' => Ok(PlayingCard::Four),
            '3' => Ok(PlayingCard::Three),
            '2' => Ok(PlayingCard::Two),
            _ => Err(())
        };
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
    Undetermined = 0,
}

impl HandType {
    fn get_hand_type_from_hand(hand: &Vec<PlayingCard>) -> HandType {
        // Count the occurrence of the cards in the hand
        let mut card_count = IndexMap::new();
        for card in hand {
            *card_count.entry(card).or_insert(0) += 1;
        }

        card_count.sort_by(|_, a_count, _, b_count| a_count.cmp(b_count) );
        card_count.reverse();
        let(_, first_count) = card_count.iter().nth(0).unwrap();
        let second_cart_count = card_count.iter().nth(1);

        let mut second_count:&i32 = &0;
        if  second_cart_count.is_some() {
            (_, second_count) = second_cart_count.unwrap();
        }


        match first_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                match second_count {
                    2 => HandType::FullHouse,
                    1 => HandType::ThreeOfAKind,
                    _ => HandType::Undetermined
                }
            }
            2 => {
                match second_count {
                    2 => HandType::TwoPair,
                    1 => HandType::OnePair,
                    _ => HandType::Undetermined
                }
            }
            1 => HandType::HighCard,
            _ => HandType::Undetermined
        }
    }
}

type Hand = Vec<PlayingCard>;
type Bid = i32;

struct Play {
    hand: Hand,
    hand_type: HandType,
    bid: Bid,
}

fn parse_line(line: String) -> Play {
    let mut split_line = line.split_ascii_whitespace();
    assert_eq!(split_line.clone().count(), 2, "Split failed - it doesn't have 2 entries");

    let mut hand = Hand::new();
    for c in split_line.next().unwrap().chars() {
        if let Ok(card) = PlayingCard::get_card_type_from_char(c){
            hand.push(card);
        } else {
            assert!(false, "Failed to parse playing card {}", c);
        }
    }

    let hand_type = HandType::get_hand_type_from_hand(&hand);

    Play {
        hand,
        hand_type,
        bid: split_line.next().unwrap().parse::<i32>().unwrap()
    }
}

fn play_cmp(a: &Play, b: &Play) -> Ordering {
    let res = a.hand_type.cmp(&b.hand_type);
    match res {
        Ordering::Equal => {
            for i in 0..5 {
                let res_b = a.hand.iter().nth(i).unwrap().cmp(b.hand.iter().nth(i).unwrap());
                if res_b == Ordering::Equal {
                    continue;
                } else {
                    return res_b;
                }
            }
            Ordering::Equal
        },
        _ => { return res; }
    }
}

pub fn run() {
    let mut all_plays = vec![];

    if let Ok(lines) = read_lines("./inputs/day7/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                all_plays.push(parse_line(ip));
            }
        }
    }

    all_plays.sort_by(|a,b| play_cmp(a,b));

    let mut winnings = 0;
    for (rank, play) in all_plays.iter().enumerate() {
        println!("rank={}, bid={}", rank, play.bid);
        winnings += (rank+1) as i32 * play.bid;
    }

    println!("{}", winnings);
}