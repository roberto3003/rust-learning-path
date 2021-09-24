fn main() {
    let mut s = String::new();
    s.push_str("foo");
    s.push_str("bar");
    println!("{}", s);

    let data = "initial content";
    let mut s = data.to_string();

    s.push('s');
    println!("{}", s);

    let s1 = "Hello, ".to_string();
    let s2 = "World".to_string();
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = "tic".to_owned();
    let s2 = "tac".to_owned();
    let s3 = "toe".to_owned();
    let s4 = format!("{} {} {}", s1, s2, s3);
    println!("{}", s4);

    let s = String::from("initial contents");
    for c in s.chars() {
        println!("{}", c);
    }

    let s = "initial contents".to_owned();
    for b in s.bytes() {
        println!("{}", b);
    }
}
