#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(1 + 1),
    }
}

fn main() {
    println!("{:#?}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //    let some_u8_value = 0u8;
    //    match some_u8_value {
    //        1 => println!("one"),
    //        3 => println!("three"),
    //        5 => println!("five"),
    //        7 => println!("seven"),
    //        _ => (),
    //    }

    let another_u8_value = Some(0u8);
    //    match another_u8_value {
    //        Some(3) => println!("three"),
    //        _ => (),
    //    }

    if let Some(3) = another_u8_value {
        println!("three");
    }

    //    let mut count = 0;
    //    match coin {
    //        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //        _ = count += 1,
    //    }
    //}

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
