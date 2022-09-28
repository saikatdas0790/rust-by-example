use std::path::Path;

fn main() {
    let path = Path::new(".");

    let _display = path.display();

    let mut new_path = path.join("a").join("b");

    new_path.push("c");
    new_path.push("myfile.tar.gz");

    new_path.set_file_name("package.tgz");

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
