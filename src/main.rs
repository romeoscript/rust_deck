use rand::{thread_rng, Rng , seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Spades", "Hearts", "Clubs", "Diamonds"];
        let values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("heres your deck: {:#?}", deck);
}
