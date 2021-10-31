struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

struct GroceryItem {
    stock: i32,
    price: f64,
}

enum Flavor {
    Apple,
    Cherry,
    Lemon,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: i32,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Apple => println!("flavor: apple"),
        Flavor::Cherry => println!("flavor: cherry"),
        Flavor::Lemon => println!("flavor: lemon"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };

    let tall = my_box.height;

    println!("The box is {:?} units tall", tall);

    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("stock: {:?}", cereal.stock);
    println!("stock: {:?}", cereal.price);

    let apple = Drink {
        flavor: Flavor::Apple,
        fluid_oz: 25,
    };

    print_drink(apple);

    let cherry = Drink {
        flavor: Flavor::Cherry,
        fluid_oz: 12,
    };

    print_drink(cherry);
}
