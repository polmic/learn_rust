mod rectangle;

use rectangle::Rectangle;

fn main() {
    let rectangle = Rectangle::new(32, 32);

    println!("{:?}", rectangle);

    println!("Area: {}", rectangle.area());
}
