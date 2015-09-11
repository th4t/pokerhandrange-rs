use cards::card::{Value};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum RangeComponent {
    Pair(Value),
    CardsSuited(Value, Value), // AJs
    CardsUnsuited(Value, Value) // AQo
}
