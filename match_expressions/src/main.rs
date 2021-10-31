fn main() {
    let some_bool = true;
    match some_bool {
        true => println!("it's true"),
        false => println!("it's false"),
    }

    let some_int = 6;
    match some_int {
        1 => println!("it's 1"),
        2 => println!("it's 2"),
        3 => println!("it's 3"),
        _ => println!("it's something else"),
    }

    let my_name = "Youo";

    match my_name {
        "Bob" => println!("That's my name"),
        "Jayson" => println!("That's not my name"),
        "Youo" => println!("Hello Youo"),
        _ => println!("Nice to meet you!"),
    }

    let is_late = true;

    match is_late {
        true => println!("You should go"),
        false => println!("You should stay longer"),
    }

    let cars = 2;
    match cars {
        1 => println!("It is one car"),
        2 => println!("There are two cars"),
        3 => println!("There are three cars"),
        _ => println!("Too many cars"),
    }
}
