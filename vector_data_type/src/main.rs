fn main() {
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    let mut fruit = Vec::new();

    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);
    println!("Fruits: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec[1]);
}
