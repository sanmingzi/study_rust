enum Coin {
    Penny,
    Nickel,
    Dime,
}

fn print_coin(coin: Coin) {
    match coin {
        Coin::Penny => { println!("this is Penny") },
        Coin::Nickel => { println!("this is Nickel") },
        Coin::Dime => { println!("this is Dime") },
    }
}

fn main() {
    print_coin(Coin::Penny);
    print_coin(Coin::Nickel);
    print_coin(Coin::Dime);
}
