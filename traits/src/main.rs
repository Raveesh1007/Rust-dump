fn main() {
    let user = User {
        name: String::from("Aryan"),
        age: 21,
    };
    println!("{}", user.summarize())
}

pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("user name is {} and age is {}", self.name, self.age);
    }
}
