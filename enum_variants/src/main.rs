//enum WebEvent {
//    WELoad,
//    WEkKeys(String, char),
//    WEClick { x: i64, y: i64 },
//}

#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}

fn main() {
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    let keys = KeyPress("Ctrl+".to_owned(), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClick(click);
    let we_key = WebEvent::WEKeys(keys);

    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );
}
