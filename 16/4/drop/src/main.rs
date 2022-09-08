struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };
    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block c");
        }
        println!("Just exited block b");
        println!("Exiting block b");
    }
    println!("Just exited block a");

    drop(_a);
    println!("end of the main function");
}
