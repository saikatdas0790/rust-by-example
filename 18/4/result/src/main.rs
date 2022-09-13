fn multiply(first_number: &str, second_number: &str) -> i32 {
    first_number.parse::<i32>().unwrap() * second_number.parse::<i32>().unwrap()
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // let tt = multiply("t", "2");
    // println!("double is {}", tt);
}
