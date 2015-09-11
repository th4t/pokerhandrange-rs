extern crate cards;
extern crate pokerhandrange;

use cards::card::{Card, Value, Suit};
use pokerhandrange::{Range, SimpleRange};

//TODO: there could be way more tests here. And unit tests in the modules.

#[test]
fn construct_range_pair() {
    let range_kk = SimpleRange::new_from_string("KK").unwrap();
    let range_qq = SimpleRange::new_from_string("QQ").unwrap();

    let range_kkqq = SimpleRange::new_from_string("KK,QQ").unwrap();

    let c1 = Card::new(Value::King, Suit::Spades);
    let c2 = Card::new(Value::King, Suit::Hearts);

    //TODO: handle stuff like KsKs? just say it is false without complaining?
    //TODO: "not a valid hand" error?

    assert_eq!(range_kk.contains((&c1, &c2)), true);
    assert_eq!(range_qq.contains((&c1, &c2)), false);

    assert_eq!(range_kkqq.contains((&c1, &c2)), true);
    let c3 = Card::new(Value::Queen, Suit::Spades);
    let c4 = Card::new(Value::Queen, Suit::Hearts);
    assert_eq!(range_kkqq.contains((&c3, &c4)), true);
}

#[test]
fn construct_range_pair_range() {
    let range = SimpleRange::new_from_string("QQ-AA").unwrap();
    let ca1 = Card::new(Value::Ace, Suit::Spades);
    let ca2 = Card::new(Value::Ace, Suit::Hearts);
    let ck1 = Card::new(Value::King, Suit::Spades);
    let ck2 = Card::new(Value::King, Suit::Hearts);
    let cq1 = Card::new(Value::Queen, Suit::Spades);
    let cq2 = Card::new(Value::Queen, Suit::Hearts);
    let cj1 = Card::new(Value::Jack, Suit::Spades);
    let cj2 = Card::new(Value::Jack, Suit::Hearts);

    assert_eq!(range.contains((&ca1, &ca2)), true);
    assert_eq!(range.contains((&ck1, &ck2)), true);
    assert_eq!(range.contains((&cq1, &cq2)), true);

    assert_eq!(range.contains((&ck1, &cq2)), false);
    assert_eq!(range.contains((&ca1, &ck2)), false);
    assert_eq!(range.contains((&cj1, &cj2)), false);

    let range2 = SimpleRange::new_from_string("55-JJ").unwrap();
    assert_eq!(range2.contains((&cq1, &cq2)), false);
    assert_eq!(range2.contains((&cj1, &cj2)), true);
}

#[test]
fn draw_cards() {
    let range_kk = SimpleRange::new_from_string("KK").unwrap();
    let drawn_cards = range_kk.draw();

    assert_eq!(range_kk.contains((&drawn_cards.0, &drawn_cards.1)), true);
}

#[test]
fn construct_range_two_cards() {
    let range_aj = SimpleRange::new_from_string("AJ").unwrap();
    let range_aq = SimpleRange::new_from_string("AQ").unwrap();
    let range_t9 = SimpleRange::new_from_string("T9").unwrap();

    let cas = Card::new(Value::Ace, Suit::Spades);
    let cah = Card::new(Value::Ace, Suit::Hearts);
    let cjh = Card::new(Value::Jack, Suit::Hearts);
    let cqh = Card::new(Value::Queen, Suit::Hearts);

    assert_eq!(range_aj.contains((&cas, &cjh)), true);
    assert_eq!(range_aj.contains((&cjh, &cas)), true); //flipped

    assert_eq!(range_aj.contains((&cah, &cjh)), true);
    assert_eq!(range_aj.contains((&cah, &cqh)), false);

    assert_eq!(range_aq.contains((&cas, &cqh)), true);
    assert_eq!(range_aq.contains((&cas, &cjh)), false);

    assert_eq!(range_t9.contains((&cas, &cqh)), false);
    assert_eq!(range_t9.contains((&cqh, &cjh)), false);
}

#[test]
fn construct_range_two_cards_plus() {
    let range_b = SimpleRange::new_from_string("J9+").unwrap();

    let cah = Card::new(Value::Ace, Suit::Hearts);
    let cjh = Card::new(Value::Jack, Suit::Hearts);
    let cts = Card::new(Value::Ten, Suit::Spades);
    let cns = Card::new(Value::Nine, Suit::Spades);
    let cnh = Card::new(Value::Nine, Suit::Hearts);

    assert_eq!(range_b.contains((&cjh, &cts)), true);
    assert_eq!(range_b.contains((&cjh, &cns)), true);
    assert_eq!(range_b.contains((&cjh, &cnh)), true);
    
    let range_u = SimpleRange::new_from_string("ATu+").unwrap();

    assert_eq!(range_u.contains((&cah, &cjh)), false);
    assert_eq!(range_u.contains((&cah, &cts)), true);

    let cqh = Card::new(Value::Queen, Suit::Hearts);
    let cqs = Card::new(Value::Queen, Suit::Spades);
    let cas = Card::new(Value::Ace, Suit::Spades);
    
    let range_s = SimpleRange::new_from_string("Q9s+").unwrap();
    assert_eq!(range_s.contains((&cqh, &cts)), false);
    assert_eq!(range_s.contains((&cqs, &cts)), true);
    assert_eq!(range_s.contains((&cqs, &cas)), false);
}

#[test]
fn construct_range_pair_plus() {
    let range = SimpleRange::new_from_string("JJ+").unwrap();

    let cjh = Card::new(Value::Jack, Suit::Hearts);
    let cjs = Card::new(Value::Jack, Suit::Spades);

    let cah = Card::new(Value::Ace, Suit::Hearts);
    let cas = Card::new(Value::Ace, Suit::Spades);

    let cth = Card::new(Value::Ten, Suit::Hearts);
    let cts = Card::new(Value::Ten, Suit::Spades);

    assert_eq!(range.contains((&cjh, &cjs)), true);
    assert_eq!(range.contains((&cah, &cas)), true);
    assert_eq!(range.contains((&cth, &cts)), false);
}

#[test]
fn construct_range_two_cards_s() {
    let range_s = SimpleRange::new_from_string("J9s").unwrap();

    let cjh = Card::new(Value::Jack, Suit::Hearts);
    let cnh = Card::new(Value::Nine, Suit::Hearts);
    let cns = Card::new(Value::Nine, Suit::Spades);

    let cts = Card::new(Value::Ten, Suit::Spades);

    assert_eq!(range_s.contains((&cjh, &cnh)), true);
    assert_eq!(range_s.contains((&cjh, &cns)), false);

    let range_u = SimpleRange::new_from_string("J9o").unwrap();
    assert_eq!(range_u.contains((&cjh, &cnh)), false);
    assert_eq!(range_u.contains((&cjh, &cns)), true);

    let range_p = SimpleRange::new_from_string("J9+").unwrap();
    assert_eq!(range_p.contains((&cjh, &cnh)), true);
    assert_eq!(range_p.contains((&cjh, &cns)), true);
    assert_eq!(range_p.contains((&cjh, &cts)), true);
}

