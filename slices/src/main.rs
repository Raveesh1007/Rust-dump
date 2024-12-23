/*fn main() {
    let string = String::from("Aryan Pachori");
    let mut first_word = String::from("");
    for i in string.chars() {
        if i == ' ' {
            break;
        }
        first_word.push_str(&i.to_string());
    }
    println!("first word: {}", first_word);
}
*/
/*fn main(){
    let name = String::from("Hello World");
    let mut space_ptr = 0;
    for i in name.chars(){
        if i == ' '{
            break;
        }
        space_ptr += 1;
    }
    let first_word: &str = &name[0..space_ptr];
    println!("First word: {}",first_word);

}
    */

fn main() {
    let string = String::from("Hello w");
    let name = &string;
    println!("Ref: {}", name);
    let b = String::from("HELLOOO");
    let a = String::from("WORLD");
    let ans = largest(a, b);
    println!("largest is : {}", ans)
}
// generics
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    } else {
        return b;
    }
}
