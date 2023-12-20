#[derive(Debug)]
enum State {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn to_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{state:?}");
            100
        }
    }
}

fn main() {
    let c = Coin::Quarter(State::Alaska);
    println!("c is worth {} cents", to_cents(&c));

    let u: Option<u8> = None;
    if let Some(smt) = u {
        println!("smt is {smt}");
    } else {
        println!("smt is None");
    }
}
