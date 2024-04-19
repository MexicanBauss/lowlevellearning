use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
#[derive(Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: u8,
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        let rank: u8 = rng.gen_range(1..=13);
        let suit: Suit = match rng.gen_range(0..=3) {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            _ => Suit::Spades,
        };
        Card {
            suit,
            rank,
        }
    }
}

fn main() {

    let mut rng = rand::thread_rng();
    let hand: Vec<Card> = (0..5).map(|_| rng.gen()).collect();
    println!("Your hand: {:?}", hand);

}
