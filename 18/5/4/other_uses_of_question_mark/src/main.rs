use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct EmptyVec;

impl Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Please use a vector with at least one element.")
    }
}

impl Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
