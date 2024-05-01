#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
struct Purse {
    bag: Vec<Coin>,
    total: u8,
}
impl Purse {
    fn new() -> Purse {
        Purse { bag: Vec::new(), total: 0 }
    }
}
impl Purse {
    fn save_coin(&mut self, coin: Coin) {
        self.total += value_in_cents(&coin);
        self.bag.push(coin);
    }
}
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1, 
        Coin::Nickel => 5,
        Coin::Dime => 10,
            Coin::Quarter => 25,
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}\nsix: {:?}\nnone: {:?}",five.expect("Not an int."), six.expect("Else"), none);

    let mut wallet = Purse::new();
    while wallet.total < 100 {
        wallet.save_coin(Coin::Nickel);
        wallet.save_coin(Coin::Penny);
        wallet.save_coin(Coin::Quarter);
        wallet.save_coin(Coin::Dime);
        println!("You have {} cents in your purse!", wallet.total);
    }

}