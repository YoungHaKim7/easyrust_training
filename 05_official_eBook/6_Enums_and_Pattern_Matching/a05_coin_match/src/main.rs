enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    let my_coin = Coin::Dime;
    println!("my coin: {}", value_in_cents(my_coin));
    let my_coin02 = Coin::Penny;
    println!("my coin: {}", value_in_cents(my_coin02));
}
