enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn coordinate() -> (i32, i32) {
    (1, 7)
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();
    println!("{:?}, {:?}", x, numbers.0); //1
    println!("{:?}, {:?}", y, numbers.1); //2
    println!("{:?}, {:?}", z, numbers.2); //3

    let (_employee, _access) = ("Jake", Access::Full);

    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);

    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Emma", 20);

    let (x, y) = coordinate();

    if y > 5 {
        println!(">5");
    } else if y == 5 {
        println!("=5")
    } else {
        println!("<5")
    }
}
