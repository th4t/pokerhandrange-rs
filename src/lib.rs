//! pokerhandrange-rs defines a Range trait and contains a SimpleRange, which
//! can be used to create a range representation with something like
//! new_from_string("AA,AK+"). The resulting SimpleRange can be used to
//! check if any two cards are covered by it or to draw sample cards from it.

extern crate rand;

extern crate cards;
extern crate holdem;
extern crate pokereval;

mod utils;
mod parse;
mod internal;

use rand::{Rng};
use std::cmp::{min, max};
use std::collections::BTreeSet;
use std::io::{Error, ErrorKind};

use cards::card::{Card};

use utils::gen_random_suits;
use parse::{parse_5_chars, parse_4_chars, parse_3_chars, parse_2_chars};
use internal::RangeComponent;

/// Any range should be able to get checked whether a hand can be in it and to draw a random card sample.
pub trait Range {
    fn contains(&self, hand: (&Card, &Card)) -> bool;
    fn draw(&self) -> (Card, Card);
}

/// The simplest of range types.
pub struct SimpleRange {
    text: String,
    components: BTreeSet<RangeComponent>,
    combination_lookup: Vec<(usize, RangeComponent)>,
    combination_count: usize,
}

impl SimpleRange {
    /// Construct a range from a non-empty string of the kind "AA,AJs+". For more components, see the README.
    pub fn new_from_string(ranges: &str) -> Result<SimpleRange, Error> {
        let raw_components = ranges.split(",");
        let mut components : BTreeSet<RangeComponent> = BTreeSet::new();

        //parse every single range component
        for component in raw_components {
            let chars : Vec<char> = component.chars().collect();
            let new_components = match chars.len() {
                5 => { try!(parse_5_chars(chars)) },
                4 => { try!(parse_4_chars(chars)) },
                3 => { try!(parse_3_chars(chars)) },
                2 => { try!(parse_2_chars(chars)) },
                _ => { return Err(Error::new(ErrorKind::Other,
                  format!("Too many characters in pattern '{}'", component))); }
            };

            for new_component in new_components {
                components.insert(new_component);
            }
        }

        if components.len() == 0 {
            return Err(Error::new(ErrorKind::Other,
                "Zero length range string not allowed"));
        }


        //construct an array with a running sum of combinations
        //this is used later to draw cards from the range, weighted by their
        //probability weighted by number of possible combinations
        // pair: 6 combos
        // suited cards:    4
        //   one possibility for each suit
        // unsuited cards:  4 * 3
        //   4 suits for first card, for each 3 other suits for second
        let mut combination_count = 0;
        let mut combination_lookup : Vec<(usize, RangeComponent)> = Vec::new();

        for component in components.iter() {
            match *component {
                RangeComponent::Pair(_) => { combination_count+=6; },
                RangeComponent::CardsSuited(_, _) => { combination_count+=4; },
                RangeComponent::CardsUnsuited(_, _) => { combination_count+=12; },
            }
            combination_lookup.push( (combination_count, component.clone() ) );
        }

        Ok(SimpleRange {
            text: String::from(ranges),
            components: components,
            combination_lookup: combination_lookup,
            combination_count: combination_count,
        })
    }

    pub fn get_range_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_component_count(&self) -> usize {
        self.components.len()
    }
}

impl Range for SimpleRange {
    /// Checks whether a card is in the range.
    fn contains(&self, hand: (&Card, &Card)) -> bool {
        let value_greater = max(hand.0.value, hand.1.value);
        let value_lesser = min(hand.0.value, hand.1.value);

        let mut ita = self.components.iter();

        let result = if hand.0.value == hand.1.value {
            ita.any(|&x| match x {
                RangeComponent::Pair(val) => val == value_greater,
                _ => false
            })
        } else if hand.0.suit == hand.1.suit {
            ita.any(|&x| match x {
                RangeComponent::CardsSuited(val_g, val_l) => val_g == value_greater && val_l == value_lesser,
                _ => false
            })
        } else {
            //filter for non-suited
            ita.any(|&x| match x {
                RangeComponent::CardsUnsuited(val_g, val_l) => val_g == value_greater && val_l == value_lesser,
                _ => false
            })
        };

        result
    }

    /// Draw a hand from the range, weighted by their combinatoric probability. Another option would be uniform probability for any component.
    fn draw(&self) -> (Card, Card) {
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0, self.combination_count); // a random number that falls into the range of enumerated combinations

        let mut speculative_component : Option<RangeComponent> = None;
        for item in self.combination_lookup.iter() {
            let (cummulative_combination_count, component) = *item;

            if n < cummulative_combination_count {
                speculative_component = Some(component);
                break;
            }
        }

        //TODO: this might be a bit crude (the unwrapping)
        let range_component = speculative_component.unwrap();
        let cards = match range_component {
            RangeComponent::Pair(val) => {
                let (suit_one, suit_two) = gen_random_suits(&mut rng);

                let c1 = Card::new(val, suit_one);
                let c2 = Card::new(val, suit_two);
                (c1, c2)
            },
            RangeComponent::CardsSuited(val_g,val_l) => {
                let (suit_one, _) = gen_random_suits(&mut rng);
                let c1 = Card::new(val_g, suit_one);
                let c2 = Card::new(val_l, suit_one);
                (c1, c2)
            },
            RangeComponent::CardsUnsuited(val_g,val_l) => {
                let (suit_one, suit_two) = gen_random_suits(&mut rng);
                let c1 = Card::new(val_g, suit_one);
                let c2 = Card::new(val_l, suit_two);
                (c1, c2)
            },
        };
        cards
    }
}
