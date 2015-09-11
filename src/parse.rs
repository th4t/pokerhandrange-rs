use std::collections::BTreeSet;
use std::cmp::{min, max};
use std::io::{Error, ErrorKind};

use cards::card::{Value};

use super::internal::RangeComponent;
use super::utils::{is_suited, is_unsuited, is_plus};
use super::utils::{card_value_from_char, CardValueIterator};

//TODO: is there something simpler?
pub type TreeOrError = Result<BTreeSet<RangeComponent>, Error>;

pub fn parse_5_chars(chars: Vec<char>) -> TreeOrError {
    if chars[2] != '-' {
        //TODO: is there something simpler?
        return Err(Error::new(ErrorKind::Other,
            format!("No dash in expected pattern '{:?}'", chars)));
    }

    if chars[0] != chars[1] || chars[3] != chars[4] {
        return Err(Error::new(ErrorKind::Other,
            format!("Not a range of pairs '{:?}'", chars)));
    }

    let value_a = card_value_from_char(chars[0]);
    let value_b = card_value_from_char(chars[3]);

    let value_greater = max(value_a, value_b);
    let value_lesser = min(value_a, value_b);

    let mut components = BTreeSet::new();

    for value in CardValueIterator::new(value_lesser, value_greater) {
        components.insert(RangeComponent::Pair(value));
    }
    
    Ok(components)
}

pub fn parse_4_chars(chars: Vec<char>) -> TreeOrError {
    let suited = is_suited(chars[2]);
    let unsuited = is_unsuited(chars[2]);
    let plus = is_plus(chars[3]);

    if !plus {
        return Err(Error::new(ErrorKind::Other,
            format!("Something is awry with this thing - no plus '{:?}'", chars)));
    }

    if !suited && !unsuited {
        return Err(Error::new(ErrorKind::Other,
            format!("Something is awry with this thing '{:?}'", chars)));
    }

    let mut components = BTreeSet::new();

    let first_char = chars[0];
    let second_char = chars[1];

    if first_char == second_char {
        return Err(Error::new(ErrorKind::Other,
            "This does not make any sense (JJs+ or similar)"));
    }

    let value_a = card_value_from_char(first_char);
    let value_b = card_value_from_char(second_char);

    let value_greater = max(value_a, value_b);
    let value_lesser = min(value_a, value_b);

    //TODO: putdis in an other function
    for val in CardValueIterator::new(value_lesser, value_greater) {
        //TODO: this is not in suited
        //TODO: exclude consideration of pairs completely?
        if value_greater == val {
            break;
        }

        //TODO: unsuited implied by suited?
        if suited {
            components.insert(RangeComponent::CardsSuited(
                value_greater, val)
            );
        }
        if unsuited {
            components.insert(RangeComponent::CardsUnsuited(
                value_greater, val)
            );
        }
    }

    Ok(components)
}

pub fn parse_3_chars(chars: Vec<char>) -> TreeOrError {
    let suited = is_suited(chars[2]);
    let unsuited = is_unsuited(chars[2]);
    let plus = is_plus(chars[2]);

    if !plus && !suited && !unsuited {
        return Err(Error::new(ErrorKind::Other,
            format!("Something is awry with this thing - no modifiers '{:?}'", chars)));
    }

    let value_a = card_value_from_char(chars[0]);
    let value_b = card_value_from_char(chars[1]);

    let value_greater = max(value_a, value_b);
    let value_lesser = min(value_a, value_b);

    let mut components = BTreeSet::new();

    if value_greater == value_lesser {
        if !plus {
            return Err(Error::new(ErrorKind::Other,
                format!("This does not make any sense (like JJs)")));
        }

        for val in CardValueIterator::new(value_greater, Value::Ace) {
            components.insert(RangeComponent::Pair(val));
        }
    } else if !plus {
        if suited {
            components.insert(RangeComponent::CardsSuited(
                value_greater, value_lesser)
            );
        } else if unsuited {
            components.insert(RangeComponent::CardsUnsuited(
                value_greater, value_lesser)
            );
        }
    } else {
        for val in CardValueIterator::new(value_lesser, value_greater) {
            //TODO: this is not in suited
            //TODO: exclude consideration of pairs completely?
            if value_greater == val {
                break;
            }
    
            //TODO: unsuited implied by suited?
            components.insert(RangeComponent::CardsSuited(
                value_greater, val)
            );
            components.insert(RangeComponent::CardsUnsuited(
                value_greater, val)
            );
        }
    }

    Ok(components)
}

pub fn parse_2_chars(chars: Vec<char>) -> TreeOrError {
    let mut components = BTreeSet::new();

    let first_char = chars[0];
    let second_char = chars[1];

    if first_char == second_char {
        let value = card_value_from_char(first_char);
        components.insert(RangeComponent::Pair(value));
    } else {
        let value_a = card_value_from_char(first_char);
        let value_b = card_value_from_char(second_char);

        let value_greater = max(value_a, value_b);
        let value_lesser = min(value_a, value_b);

        //find out greater one
        components.insert(RangeComponent::CardsSuited(
            value_greater, value_lesser)
        );
        components.insert(RangeComponent::CardsUnsuited(
            value_greater, value_lesser)
        );
    }
    Ok(components)
}

