fn main() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref val => println!("Got a reference to a value: {:?}", val),
    }

    match mut_value {
        ref mut val => {
            *val += 7;
            println!("We mutably borrowed the value: {:?}", val);
        }
    }
}
