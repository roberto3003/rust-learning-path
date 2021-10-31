fn main() {
    let mut i = 3;
    loop {
        println!("{:?}", i);
        if i == 0 {
            break;
        }
        i = i - 1;
    }
    println!("done!");

    let mut i = 1;
    loop {
        println!("{:?}", i);
        if i == 4 {
            break;
        }
        i = i + 1;
    }
    println!("done!");

    let mut i = 1;
    while i <= 3 {
        println!("{:?}", i);
        i = i + 1;
    }
    println!("done!");

    let mut i = 5;
    while i >= 1 {
        println!("{:?}", i);
        i = i - 1;
    }
    println!("done!");
}
