fn main() {
    let age = 20;
    if age >= 21 {
        println!("Ok to purchase")
    } else {
        println!("Cannot purchase")
    }

    let is_friend = false;

    if is_friend == true {
        println!("hello")
    } else {
        println!("goodbye")
    }

    let num = 5;

    if num == 5 {
        println!("The number is correct")
    } else if num > 5 {
        println!("The number is too big!")
    } else {
        println!("The number is too small.")
    }
}
