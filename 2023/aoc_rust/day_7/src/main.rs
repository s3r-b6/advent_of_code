use std::cmp::Ordering;

// HandType, original str, bid amount
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Num(u8),
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum HandType {
    NKind5(Card),
    NKind4(Card),
    FHouse(Card, Card),
    NKind3(Card),
    NPair2(Card, Card),
    NPair1(Card),
    HiCard(Card),
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Hand {
    kind: HandType,
    cards: String,
    bid: u32,
    joker: bool, // I don't really like this
}

impl Card {
    fn value_p2(&self) -> u8 {
        match &self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::T => 10,
            Card::Num(n) => *n,
            Card::J => 1,
        }
    }

    fn value_p1(&self) -> u8 {
        match &self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::Num(n) => *n,
        }
    }
}

impl Hand {
    fn value(&self) -> u8 {
        match &self.kind {
            HandType::NKind5(_) => 7,
            HandType::NKind4(_) => 6,
            HandType::FHouse(_, _) => 5,
            HandType::NKind3(_) => 4,
            HandType::NPair2(_, _) => 3,
            HandType::NPair1(_) => 2,
            HandType::HiCard(_) => 1,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_val = self.value();
        let other_val = other.value();

        if self_val != other_val {
            return self_val.cmp(&other_val);
        }

        // This is for part2 compatibility, if the joker is enabled,
        // the joker is the card with lowest value
        if !self.joker {
            // If they are not equal, return the cmp between the
            // first non equal chars
            let mut ch0 = '\0';
            let mut ch1 = '\0';

            let mut idx = 0;
            while ch0 == '\0' || ch0 == ch1 {
                ch0 = self.cards[idx..idx + 1].chars().next().unwrap();
                ch1 = other.cards[idx..idx + 1].chars().next().unwrap();
                idx += 1;
            }

            Card::from(&ch0)
                .value_p1()
                .cmp(&Card::from(&ch1).value_p1())
        } else {
            let mut ch0 = '\0';
            let mut ch1 = '\0';

            let mut idx = 0;
            while ch0 == '\0' || ch0 == ch1 {
                ch0 = self.cards[idx..idx + 1].chars().next().unwrap();
                ch1 = other.cards[idx..idx + 1].chars().next().unwrap();
                idx += 1;
            }

            Card::from(&ch0)
                .value_p2()
                .cmp(&Card::from(&ch1).value_p2())
        }
    }
}

impl From<&char> for Card {
    fn from(card: &char) -> Self {
        match card {
            '0'..='9' => Card::Num(*card as u8 - '0' as u8),
            'A' => Card::A,
            'K' => Card::K,
            'T' => Card::T,
            'Q' => Card::Q,
            'J' => Card::J,
            _ => unreachable!("{} is not a valid card!", card),
        }
    }
}

impl From<(HandType, &str, u32, bool)> for Hand {
    fn from((kind, cards, bid, joker): (HandType, &str, u32, bool)) -> Self {
        Hand {
            kind,
            cards: cards.to_string(),
            bid,
            joker,
        }
    }
}

fn cards_order(a: &char, b: &char, joker: bool) -> Ordering {
    if !joker {
        return match (a, b) {
            _ if a == b => Ordering::Equal,
            ('0'..='9', '0'..='9') => return b.cmp(a),
            ('A'..='Z', '0'..='9') => return Ordering::Less,
            ('0'..='9', 'A'..='Z') => return Ordering::Greater,
            ('A'..='Z', 'A'..='Z') => match (a, b) {
                ('A', _) => return Ordering::Less,
                (_, 'A') => return Ordering::Greater,
                ('K', _) => return Ordering::Less,
                (_, 'K') => return Ordering::Greater,
                ('Q', _) => return Ordering::Less,
                (_, 'Q') => return Ordering::Greater,
                ('J', _) => return Ordering::Less,
                (_, 'J') => return Ordering::Greater,
                ('T', _) => return Ordering::Less,
                (_, 'T') => return Ordering::Greater,
                _ => unreachable!(),
            },
            (_, _) => unreachable!(),
        };
    } else {
        return match (a, b) {
            _ if a == b => Ordering::Equal,
            ('J', _) => Ordering::Greater,
            (_, 'J') => Ordering::Less,
            ('0'..='9', '0'..='9') => return b.cmp(a),
            ('A'..='Z', '0'..='9') => return Ordering::Less,
            ('0'..='9', 'A'..='Z') => return Ordering::Greater,
            ('A'..='Z', 'A'..='Z') => match (a, b) {
                ('A', _) => return Ordering::Less,
                (_, 'A') => return Ordering::Greater,
                ('K', _) => return Ordering::Less,
                (_, 'K') => return Ordering::Greater,
                ('Q', _) => return Ordering::Less,
                (_, 'Q') => return Ordering::Greater,
                ('J', _) => return Ordering::Less,
                (_, 'J') => return Ordering::Greater,
                ('T', _) => return Ordering::Less,
                (_, 'T') => return Ordering::Greater,
                _ => unreachable!(),
            },
            (_, _) => unreachable!(),
        };
    }
}

fn main() {
    let contents = include_str!("../input.txt");

    println!("\nPART1: {}", part1(contents));
    println!("\nPART2: {}", part2(contents));
}

fn part1(contents: &str) -> u32 {
    let mut hands: Vec<Hand> = vec![];
    for l in contents.lines() {
        let mut parts_iter = l.split(' ');
        let hand_str = parts_iter.next().unwrap();
        let bid = parts_iter.next().unwrap().parse().unwrap();
        let hand = Hand {
            cards: hand_str.to_string(),
            kind: get_hand_type_p1(hand_str),
            bid,
            joker: false,
        };
        hands.push(hand);
    }

    hands.sort_by(|a, b| a.cmp(b));
    let mut acc: u32 = 0;
    let hands_len = hands.len();
    for idx in 0..hands_len {
        let h = &hands[idx];
        //println!("{} -> {:?} \n  {} * {}", h.cards, h.kind, idx + 1, h.bid);
        acc += (idx + 1) as u32 * h.bid;
    }

    acc
}

fn get_hand_type_p1(hand: &str) -> HandType {
    let mut sorted_hand: Vec<char> = hand.chars().collect::<Vec<char>>();
    sorted_hand.sort_by(|a, b| cards_order(a, b, false));

    //println!("{:?}", &sorted_hand);

    let mut five_of_kind = '\0';
    let mut four_of_kind = '\0';
    let mut three_of_kind = '\0';
    let mut pairs: Vec<char> = vec![];
    let mut prev_char = '\0';
    let highest_card = sorted_hand[0];

    let mut amount = 0;
    for ch in sorted_hand {
        if !(ch == prev_char || amount == 0) {
            amount = 1;
            prev_char = ch;
            continue;
        }

        amount += 1;
        match amount {
            1 => {}
            2 => pairs.push(ch),
            3 => {
                pairs.pop();
                three_of_kind = ch;
            }
            4 => {
                three_of_kind = '\0';
                four_of_kind = ch;
            }
            5 => {
                four_of_kind = '\0';
                five_of_kind = ch;
            }
            _ => unreachable!(),
        }

        prev_char = ch;
    }

    if five_of_kind != '\0' {
        return HandType::NKind5(Card::from(&five_of_kind));
    } else if four_of_kind != '\0' {
        return HandType::NKind4(Card::from(&four_of_kind));
    } else if three_of_kind != '\0' && pairs.len() == 1 {
        return HandType::FHouse(
            Card::from(&three_of_kind),
            Card::from(&pairs.pop().unwrap()),
        );
    } else if three_of_kind != '\0' {
        return HandType::NKind3(Card::from(&three_of_kind));
    } else if pairs.len() == 2 {
        return HandType::NPair2(
            Card::from(&pairs.pop().unwrap()),
            Card::from(&pairs.pop().unwrap()),
        );
    } else if pairs.len() == 1 {
        return HandType::NPair1(Card::from(&pairs.pop().unwrap()));
    } else {
        return HandType::HiCard(Card::from(&highest_card));
    }
}

fn part2(contents: &str) -> u32 {
    let mut hands: Vec<Hand> = vec![];
    for l in contents.lines() {
        let mut parts_iter = l.split(' ');
        let hand_str = parts_iter.next().unwrap();
        let bid = parts_iter.next().unwrap().parse().unwrap();
        let hand = Hand {
            cards: hand_str.to_string(),
            kind: get_hand_type_p2(hand_str),
            bid,
            joker: true,
        };
        hands.push(hand);
    }

    hands.sort_by(|a, b| a.cmp(b));
    let mut acc: u32 = 0;
    let hands_len = hands.len();
    for idx in 0..hands_len {
        let h = &hands[idx];
        println!("{} -> {:?} \n  {} * {}", h.cards, h.kind, idx + 1, h.bid);
        acc += (idx + 1) as u32 * h.bid;
    }

    acc
}

fn get_hand_type_p2(hand: &str) -> HandType {
    let mut sorted_hand: Vec<char> = hand.chars().collect::<Vec<char>>();
    sorted_hand.sort_by(|a, b| cards_order(a, b, true));

    let mut five_of_kind = &'\0';
    let mut four_of_kind = &'\0';
    let mut three_of_kind = &'\0';
    let mut pairs: Vec<&char> = vec![];
    let mut prev_char = &'\0';
    let highest_card = sorted_hand[0];

    let mut j_amount = 0;
    let mut amount = 0;
    for ch in &sorted_hand {
        if ch == &'J' {
            j_amount += 1;
            continue;
        }

        if !(ch == prev_char || amount == 0) {
            amount = 1;
            prev_char = ch;
            continue;
        }

        amount += 1;

        match amount {
            1 => {}
            2 => pairs.push(ch),
            3 => {
                pairs.pop();
                three_of_kind = &ch;
            }
            4 => {
                three_of_kind = &'\0';
                four_of_kind = &ch;
            }
            5 => {
                four_of_kind = &'\0';
                five_of_kind = &ch;
            }
            _ => unreachable!(),
        }

        if prev_char != &'J' {
            prev_char = ch;
        }
    }

    println!("{:?}", &sorted_hand);
    println!("hi{:?}", &highest_card);
    println!("Prev_ch:{}", prev_char);
    println!("J:{}", j_amount);

    if five_of_kind != &'\0' {
        return HandType::NKind5(Card::from(five_of_kind));
    } else if four_of_kind != &'\0' {
        if j_amount == 1 {
            return HandType::NKind5(Card::from(four_of_kind));
        }

        return HandType::NKind4(Card::from(four_of_kind));
    } else if three_of_kind != &'\0' && pairs.len() == 1 {
        if j_amount == 2 {
            return HandType::NKind5(Card::from(three_of_kind));
        }

        return HandType::FHouse(Card::from(three_of_kind), Card::from(pairs.pop().unwrap()));
    } else if three_of_kind != &'\0' {
        if j_amount == 1 {
            return HandType::NKind4(Card::from(three_of_kind));
        } else if j_amount == 2 {
            return HandType::NKind5(Card::from(three_of_kind));
        }

        return HandType::NKind3(Card::from(three_of_kind));
    } else if pairs.len() == 2 {
        if j_amount == 1 {
            return HandType::FHouse(
                Card::from(pairs.pop().unwrap()),
                Card::from(pairs.pop().unwrap()),
            );
        } else if j_amount == 2 {
            return HandType::NKind4(Card::from(pairs.pop().unwrap()));
        } else if j_amount == 3 {
            return HandType::NKind5(Card::from(pairs.pop().unwrap()));
        }

        return HandType::NPair2(
            Card::from(pairs.pop().unwrap()),
            Card::from(pairs.pop().unwrap()),
        );
    } else if pairs.len() == 1 {
        if j_amount == 1 {
            return HandType::NKind3(Card::from(pairs.pop().unwrap()));
        } else if j_amount == 2 {
            return HandType::NKind4(Card::from(pairs.pop().unwrap()));
        } else if j_amount == 3 {
            return HandType::NKind5(Card::from(pairs.pop().unwrap()));
        }

        return HandType::NPair1(Card::from(pairs.pop().unwrap()));
    } else {
        if j_amount == 1 {
            return HandType::HiCard(Card::from(&highest_card));
        } else if j_amount == 1 {
            return HandType::NPair1(Card::from(&highest_card));
        } else if j_amount == 2 {
            return HandType::NKind3(Card::from(&highest_card));
        } else if j_amount == 3 {
            return HandType::NKind4(Card::from(&highest_card));
        } else if j_amount == 4 {
            return HandType::NKind5(Card::from(&highest_card));
        } else if j_amount == 5 {
            return HandType::NKind5(Card::from(&'J'));
        }

        return HandType::HiCard(Card::from(&highest_card));
    }
}
