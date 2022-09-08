use std::{
    io::{BufRead, Result},
    iter::{Chain, Cycle},
    vec::IntoIter,
};

fn parse_csv_document_1<R: BufRead>(src: R) -> Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| line.map(|line| line.split(',').map(|s| s.trim().to_string()).collect()))
        .collect()
}

fn parse_csv_document_2(src: impl BufRead) -> Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| line.map(|line| line.split(',').map(|s| s.trim().to_string()).collect()))
        .collect()
}

fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> Cycle<Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(v3.next(), Some(1));
    assert_eq!(v3.next(), Some(2));
    assert_eq!(v3.next(), Some(3));
    assert_eq!(v3.next(), Some(4));
    assert_eq!(v3.next(), Some(5));
    println!("all done!");

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}
