mod rectangle;

use rectangle::Rectangle;

fn main() {
    let r1 = Rectangle::new(10, 10);
    let r2 = Rectangle::new(100, 100);

    println!("r1: {:?} - r2: {:?}", r1, r2);
    println!("r1.area: {} - r2.area: {}", r1.area(), r2.area());
    println!("can r1 hold r2: {}", r1.can_hold(&r2));
    println!("can r2 hold r1: {}", r2.can_hold(&r1));

    println!();

    let s1 = Rectangle::square(100);
    let s2 = Rectangle::square(100);
    println!("s1: {:?} - s2: {:?}", s1, s2);
    println!("can s1 hold s2: {}", s1.can_hold(&s2));
}
