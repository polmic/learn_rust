#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rectangle = Rectangle {
        width: 32,
        height: 32
    };
    
    println!("rectangle: {:?}", rectangle);

    println!("Area: {}", area(rectangle));
}
