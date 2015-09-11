use rand::{Rng, ThreadRng};
use cards::card::{Value, Suit};

//TODO: this could go into the cards crate
pub struct CardValueIterator {
    current: Value,
    maximal: Value,
    finished_flag: bool,
}

impl CardValueIterator {
    /// An iterator that goes from the first value to the second value inclusive.
    pub fn new(value_lesser: Value, value_greater: Value) -> CardValueIterator {
        if value_lesser > value_greater {
            panic!("First value is greater than the second one");
        }

        CardValueIterator {
            current: value_lesser,
            maximal: value_greater,
            finished_flag: false,
        }
    }
}

impl Iterator for CardValueIterator {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        if self.finished_flag {
            return None;
        }

        if self.current == self.maximal {
            self.finished_flag = true;
        }
        let returning = self.current;

        match next_card_value(&self.current) {
            Some(x) => {self.current = x},
            None => {self.finished_flag = true}
        }

        return Some(returning);
    }
}

//TODO: put this function elsewhere
pub fn card_value_from_char(c: char) -> Value {
    match c {
        '2' => Value::Two,
        '3' => Value::Three,
        '4' => Value::Four,
        '5' => Value::Five,
        '6' => Value::Six,
        '7' => Value::Seven,
        '8' => Value::Eight,
        '9' => Value::Nine,
        'T' => Value::Ten,
        'J' => Value::Jack,
        'Q' => Value::Queen,
        'K' => Value::King,
        'A' => Value::Ace,
        _ => panic!("Unrecognized value character")
    }
}

pub fn next_card_value(cv: &Value) -> Option<Value> {
    match *cv {
         Value::Two => Some(Value::Three),
         Value::Three => Some(Value::Four),
         Value::Four => Some(Value::Five),
         Value::Five => Some(Value::Six),
         Value::Six => Some(Value::Seven),
         Value::Seven => Some(Value::Eight),
         Value::Eight => Some(Value::Nine),
         Value::Nine => Some(Value::Ten),
         Value::Ten => Some(Value::Jack),
         Value::Jack => Some(Value::Queen),
         Value::Queen => Some(Value::King),
         Value::King => Some(Value::Ace),
         _ => None
    }
}

pub fn is_plus(c: char) -> bool {
    c == '+'
}

pub fn is_suited(c: char) -> bool {
    c == 's'
}

pub fn is_unsuited(c: char) -> bool {
    c == 'o' ||  c == 'u'
}

/// Generates two distinct suits.
pub fn gen_random_suits(rng: &mut ThreadRng) -> (Suit, Suit) {
    let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

    let n1 = rng.gen_range(0,4);
    let n2 = rng.gen_range(0,3);

    let suit1 = suits[n1];
    let suit2 = if n2 >= n1 {
        suits[n2+1]
    } else {
        suits[n2]
    };

    (suit1, suit2)
}

