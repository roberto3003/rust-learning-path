use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_owned(), 10);
    scores.insert("Yellow".to_owned(), 50);
    scores.insert("Blue".to_owned(), 25);

    scores.entry("Yellow".to_owned()).or_insert(50);
    scores.entry("Green".to_owned()).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams = vec!["Blue".to_owned(), "Yellow".to_owned()];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let team_name = "Blue".to_owned();
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
