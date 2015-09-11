extern crate rand;

extern crate cards;
extern crate holdem;
extern crate pokereval;
extern crate pokerhandrange;

use rand::{Rng};
use pokerhandrange::{Range, SimpleRange};
use cards::deck::Deck;
use cards::card::Card;

//use holdem::{HandRankClass, hand_rank_to_class};

const EVALUATION_COUNT : usize = 20; //drawing cards from the ranges
const DRAW_TRIES : usize = 20; //until we give up and retry
const DEALING_COUNT : usize = 200; //dealing community cards and evaluating

fn main() {
    let range_one = SimpleRange::new_from_string("QQ,AA").unwrap();
    let range_two = SimpleRange::new_from_string("KK").unwrap();

    let mut rng = rand::thread_rng();
    let mut deck = Deck::new_shuffled();

    let ranges = [range_one, range_two];
    let mut scores = (0, 0, 0, 0); //third: draw, last: collisions

    //switch "hand to draw first" each evaluation
    let mut turn = rng.gen_range(0, ranges.len());;

    for _ in 0..EVALUATION_COUNT {
        let other_turn = (turn+1)%ranges.len();
        let cards_one = ranges[turn].draw();

        // draw second hand up to x times, so it does not overlap with hand1 cards
        //TODO: what if the ranges overlap too much?
        // a narrower range can cripple a broader range this way, maybe disregard similar cards?
        let mut cards_two = ranges[other_turn].draw();
        let mut collision_flag = true;
        for _ in 0..DRAW_TRIES {
            // are we sharing cards?
            if cards_two.0 != cards_one.0 && cards_two.0 != cards_one.1 && cards_two.1 != cards_one.0 && cards_two.1 != cards_one.1 {
                collision_flag = false;
                break; //yeii no collisions anymore
            }
            cards_two = ranges[other_turn].draw(); //draw new cards
        }

        if collision_flag {
            println!("There was a hard to resolve collision.");
            scores.3 += 1;
            continue; //TODO: change turn order?
        }

        let cards = if turn == 0 {
             [cards_one, cards_two]
        } else {
             [cards_two, cards_one]
        };

        //deal y times, see who is the winner each time (evaluate)
        for _ in 0..DEALING_COUNT {
            deck.reset_shuffled();

            let mut com_cards : Vec<Card> = Vec::new();

            while com_cards.len() < 5 {
                let card = deck.draw().ok().unwrap();

                if card != cards[0].0 && card != cards[0].1 && card != cards[1].0 && card != cards[1].1 {
                    com_cards.push(card);
                }
            }

            //evaluate
            let cards_one = [&com_cards[0], &com_cards[1], &com_cards[2], &com_cards[3], &com_cards[4], &cards[0].0, &cards[0].1];
            let score_one = pokereval::eval_7cards(&cards_one);
            let cards_two = [&com_cards[0], &com_cards[1], &com_cards[2], &com_cards[3], &com_cards[4], &cards[1].0, &cards[1].1];
            let score_two = pokereval::eval_7cards(&cards_two);

            /*
            println!("Hand one: {:?}", cards_one);
            println!("Hand two: {:?}", cards_two);
            println!("{:?} against {:?}", hand_rank_to_class(&score_one), hand_rank_to_class(&score_two));
            */

            if score_one > score_two {
                scores.0 += 1;
            } else if score_two > score_one {
                scores.1 += 1;
            } else {
                scores.2 += 1;
            }
            //println!("{} vs {}", score_one, score_two);
        }
        
        // next hand's turn
        turn = other_turn;
    }

    println!("Ranges: '{}' vs '{}'", ranges[0].get_range_text(), ranges[1].get_range_text());
    println!("Range component counts: '{}' vs '{}'", ranges[0].get_component_count(), ranges[1].get_component_count());
    println!("Result: {} vs {}", scores.0, scores.1);
    println!("Draws : {}", scores.2);

    let total : f64 = (scores.0+scores.1+scores.2) as f64;
    let percent_one : f64 = (scores.0 as f64)/total;
    let percent_two : f64 = (scores.1 as f64)/total;
    println!("Percent: {} vs {}", percent_one, percent_two);

    println!("Unresolved collisions: {}", scores.3);
}
