use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    i64,
    io::{self, BufRead},
    path::Path,
    sync::Arc,
    usize,
};

fn main() {
    let mut card_map_part_one: HashMap<char, (Card, i64)> = HashMap::new();
    let mut card_map_part_two: HashMap<char, (Card, i64)> = HashMap::new();

    populate_map_part_one(&mut card_map_part_one);
    populate_map_part_two(&mut card_map_part_two);

    let lines: Vec<String> = read_lines("in_real.txt")
        .expect("can't read file")
        .flatten()
        .collect();

    let hands_part_one: Vec<Hand> = lines
        .iter()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            Hand::new(
                Arc::from(split[0].chars().collect::<Vec<char>>()),
                split[1].parse().unwrap(),
                &card_map_part_one,
            )
        })
        .collect::<Vec<Hand>>()
        .into();

    let hands_part_two: Vec<Hand> = lines
        .iter()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            Hand::new(
                Arc::from(split[0].chars().collect::<Vec<char>>()),
                split[1].parse().unwrap(),
                &card_map_part_two,
            )
        })
        .collect::<Vec<Hand>>()
        .into();

    println!("Solution Part 1: {}", part_one(hands_part_one));
    println!("Solution Part 2: {}", part_two(hands_part_two));
}

fn part_one(hands: Vec<Hand>) -> i64 {
    let mut values: Vec<Hand> = hands.clone();
    let mut acc = 0;

    values.sort();

    for (i, hand) in values.iter().enumerate() {
        acc += (i as i64 + 1) * hand.bid
    }

    acc
}

fn part_two(hands: Vec<Hand>) -> i64 {
    let mut values: Vec<Hand> = hands.clone();
    let mut acc = 0;

    values = values
        .iter()
        .map(|hand| {
            let j_count = hand.cards.iter().filter(|&x| x.0 == Card::J).count();

            let new_hand_type = if j_count > 0 {
                HandType::new_handtype(j_count, hand.hand_name)
            } else {
                hand.hand_name
            };

            let mut new_hand = hand.clone();

            new_hand.hand_name = new_hand_type;

            if j_count > 0 {
                println!(
                    "Cards: {:?} | Old Hand: {:?} | New Hand: {:?}",
                    hand.cards.iter().map(|x| x.0).collect::<Vec<Card>>(),
                    hand.hand_name,
                    new_hand_type
                );
            }

            new_hand
        })
        .collect();

    values.sort();

    for (i, hand) in values.iter().enumerate() {
        acc += (i as i64 + 1) * hand.bid
    }

    acc
}

#[derive(Debug, Eq, Clone)]
struct Hand {
    cards: Arc<[(Card, i64)]>,
    hand_name: HandType,
    bid: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    fn new_handtype(j_count: usize, previous: HandType) -> Self {
        match (previous, j_count) {
            (Self::HighCard, _) => Self::OnePair,
            (Self::OnePair, _) => Self::ThreeOfAKind,
            (Self::TwoPair, 1) => Self::FullHouse,
            (Self::TwoPair, 2) => Self::FourOfAKind,
            (Self::ThreeOfAKind, _) => Self::FourOfAKind,
            (Self::FullHouse, _) => Self::FiveOfAKind,
            (Self::FourOfAKind, _) => Self::FiveOfAKind,
            (_, _) => previous,
        }
    }
}

impl Hand {
    fn new(cards: Arc<[char]>, bid: i64, map: &HashMap<char, (Card, i64)>) -> Self {
        let hand_name = Hand::get_hand_name(cards.clone());
        let cards_from_enum = cards.clone().iter().map(|card| map[card]).collect();
        Self {
            cards: cards_from_enum,
            hand_name,
            bid,
        }
    }

    fn get_hand_name(hand: Arc<[char]>) -> HandType {
        let mut map: HashMap<char, i64> = HashMap::new();

        for &char in hand.iter() {
            let count = map.entry(char).or_insert(0);
            *count += 1;
        }

        let mut values: Vec<i64> = map.values().copied().collect();
        values.sort();

        match values[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [..] => panic!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_ord = (self.hand_name as isize).cmp(&(other.hand_name as isize));

        if !hand_ord.is_eq() {
            return hand_ord;
        }

        for i in 0..self.cards.len() {
            let self_card = &self.cards[i];
            let other_card = &other.cards[i];

            let cmp = self_card.1.cmp(&other_card.1);

            if !cmp.is_eq() {
                return cmp;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_name == other.hand_name
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn populate_map_part_one(map: &mut HashMap<char, (Card, i64)>) {
    map.insert('2', (Card::Two, 1));
    map.insert('3', (Card::Three, 2));
    map.insert('4', (Card::Four, 3));
    map.insert('5', (Card::Five, 4));
    map.insert('6', (Card::Six, 5));
    map.insert('7', (Card::Seven, 6));
    map.insert('8', (Card::Eight, 7));
    map.insert('9', (Card::Nine, 8));
    map.insert('T', (Card::T, 9));
    map.insert('J', (Card::J, 10));
    map.insert('Q', (Card::Q, 11));
    map.insert('K', (Card::K, 12));
    map.insert('A', (Card::A, 13));
}

fn populate_map_part_two(map: &mut HashMap<char, (Card, i64)>) {
    map.insert('J', (Card::J, 0));
    map.insert('2', (Card::Two, 1));
    map.insert('3', (Card::Three, 2));
    map.insert('4', (Card::Four, 3));
    map.insert('5', (Card::Five, 4));
    map.insert('6', (Card::Six, 5));
    map.insert('7', (Card::Seven, 6));
    map.insert('8', (Card::Eight, 7));
    map.insert('9', (Card::Nine, 8));
    map.insert('T', (Card::T, 9));
    map.insert('Q', (Card::Q, 10));
    map.insert('K', (Card::K, 11));
    map.insert('A', (Card::A, 12));
}
