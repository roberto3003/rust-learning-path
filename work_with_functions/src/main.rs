fn goodbye(message: &str) -> bool {
    println!("\n{}", message);
    return true;
}

fn divide_by_5(num: u32) -> u32 {
    num / 5
}

fn main() {
    let formal = "Formal: Good bye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);

    let num = 25;
    println!("\n{} divided by 5 = {}", num, divide_by_5(25));
}
