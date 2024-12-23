struct Rect {
    width: u32,
    length: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.length * self.width;
    }
}
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(length, width) => width * length,
    }
}
fn main() {
    let rect: Rect = Rect {
        width: 10,
        length: 20,
    };
    println!("area of rectangle is {}", rect.area());

    let circle: Shape = Shape::Circle(5.0);
    let square: Shape = Shape::Square(4.0);
    let rectangle: Shape = Shape::Rectangle(2.0, 3.0);

    println!("area of circle: {}", calculate_area(circle));
    println!("area of square: {}", calculate_area(square));
    println!("area of rectangle: {}", calculate_area(rectangle));
}
