use std::io;

struct User<'a> {
    name: &'a str,
    age: u32,
}

fn main() {
    let a = String::from("Aryan");
    let b = String::from("Hello World");
    let longest = longest_str(&a, &b);
    println!("Longest string is: {}", longest);

    let mut input_name = String::new();
    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut input_name)
        .expect("Failed to read input");
    
    let input_name = input_name.trim();

    let mut input_age = String::new();
    println!("Enter your age: ");
    io:: stdin()
        .read_line(&mut input_age)
        .expect("Faiker to read input");

    let input_age = input_age.trim().parse().expect("Please enter a number");
    let user = User { name: input_name, age: input_age  };
    println!("Name is: {} and age is: {}", user.name, user.age);
}

fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
