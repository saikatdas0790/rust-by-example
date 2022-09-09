trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 32,
    };

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!(username, "rustacean".to_owned());
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(age, 32);
}
