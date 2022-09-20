use std::{sync::Arc, thread, time::Duration};

fn main() {
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }

    thread::sleep(Duration::from_secs(1));
}
