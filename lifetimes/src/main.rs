
struct User<'a>{
    name : &'a str,
    age : u32
}
fn main() {
    let a = String::from("Aryan");
    let b = String::from("Hello World");
    let longest = longest_str(&a, &b);
    println!("longest string is: {}",longest);
    let first_name = String::from("USERONE");
    let user = User{ name : &first_name, age : 19};
    println!("name is : {} and age is {}",user.name,user.age)
}
fn longest_str<'a>(a: &'a str , b : &'a str)-> &'a str{
    if a.len() > b.len(){
        return a;
    }else{
        return b;
    }
}