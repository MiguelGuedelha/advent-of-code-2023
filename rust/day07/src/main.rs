use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::Arc,
};

fn main() {
    let mut card_map: HashMap<char, Card> = HashMap::new();

    populate_map(&mut card_map);

    let lines: Vec<String> = read_lines("in.txt")
        .expect("can't read file")
        .flatten()
        .collect();

    let hands: Vec<Hand> = lines
        .iter()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            Hand::new(
                Arc::from(split[0].chars().collect::<Vec<char>>()),
                split[1].parse().unwrap(),
                &card_map,
            )
        })
        .collect::<Vec<Hand>>()
        .into();

    println!("Solution Part 1: {}", part_one(hands));
    //println!("Solution Part 2: {}", part_two(hands));
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

#[derive(Debug, Eq, Clone)]
struct Hand {
    cards: Arc<[Card]>,
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

impl Hand {
    fn new(cards: Arc<[char]>, bid: i64, map: &HashMap<char, Card>) -> Self {
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

            let cmp = self_card.cmp(other_card);

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

fn populate_map(map: &mut HashMap<char, Card>) {
    map.insert('2', Card::Two);
    map.insert('3', Card::Three);
    map.insert('4', Card::Four);
    map.insert('5', Card::Five);
    map.insert('6', Card::Six);
    map.insert('7', Card::Seven);
    map.insert('8', Card::Eight);
    map.insert('9', Card::Nine);
    map.insert('T', Card::T);
    map.insert('J', Card::J);
    map.insert('Q', Card::Q);
    map.insert('K', Card::K);
    map.insert('A', Card::A);
}
