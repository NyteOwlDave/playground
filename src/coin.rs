
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn coin_slot(coin: Coin) {
    let q : u32 = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    println!("{}", q);    
}
