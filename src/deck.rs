#![allow(non_snake_case)]

use crate::card::{card, Card};
use rand::Rng;

#[derive(Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub value: i32,
}

impl Deck {
    pub fn new() -> Self {
        let value: i32 = 0;
        let cards: Vec<Card> = Vec::new();

        // for card in &cards {
        //     value += card.value;
        // }

        Self { cards, value }
    }

    fn addCard(&mut self, card: Card) -> i32 {
        self.value += card.value;
        self.cards.push(card);

        return self.value;
    }

    pub fn hit(&mut self) {
        self.addCard(card(rand::thread_rng().gen_range(1..=10)));
    }

    pub fn start(&mut self) {
        self.hit();
        self.hit();
    }
}
