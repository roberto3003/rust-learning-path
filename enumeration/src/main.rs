enum Direction {
    Left,
    Right,
    Up,
}

enum Color {
    Blue,
    Red,
    Yellow,
    Green,
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
        Color::Green => println!("Green"),
    }
}

enum Payment {
    Cash,
    CreditCard,
    DebitCard,
}

fn main() {
    let go = Direction::Left;
    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        Direction::Up => println!("go up"),
    }

    print_color(Color::Blue);

    let some_payment = Payment::Cash;

    match some_payment {
        Payment::Cash => {
            println!("Paying with cash...");
        }
        Payment::CreditCard => {
            println!("Paying with credit card...");
        }
        Payment::DebitCard => {
            println!("Paying with debit card...");
        }
    }
}
