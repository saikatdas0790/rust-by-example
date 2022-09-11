fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary"),
        Some(drink) => println!("{}? How nice.", drink),
        None => println!("No drink? Oh well."),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
