fn main() {
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    let first = &three_nums[0];
    let second = three_nums.get(1);

    // let does_not_exist = &three_nums[3]; // Program will panic
    let does_not_exist = three_nums.get(3); // Program will not panic

    println!(
        "first number is {:?}, second number is {:?},
        no existing number is {:?}",
        first, second, does_not_exist
    );

    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    let mut fruit = Vec::new();

    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);
    println!("Fruits: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    let first = &fruit[0];
    println!("first fruit in the list is {:?}", first);
    fruit.push("Pineapple");

    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec[1]);

    for i in &index_vec {
        println!("{}", i);
    }
}
