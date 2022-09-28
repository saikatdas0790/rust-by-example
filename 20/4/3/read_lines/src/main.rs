use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    if let Ok(lines) = read_lines("./hosts") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
