enum Menu {
    Burger,
    Fries,
    Drink,
}

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}

fn main() {
    let my_num = 3;
    let is_lt_5 = if my_num < 5 { true } else { false };

    let is_lt_5 = my_num < 5;

    let my_num = 3;
    let message = match my_num {
        1 => "hello",
        _ => "goodbye",
    };

    let paid = true;
    let item = Menu::Drink;
    let dring_type = "water";
    let order_placed = match item {
        Menu::Drink => {
            if dring_type == "water" {
                true
            } else {
                false
            }
        }
        _ => true,
    };

    //secret file: admins only
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("can access? {:?}!", can_access_file);

    let value = 100;
    let is_gt_100 = value > 100;

    print_message(is_gt_100);
}
