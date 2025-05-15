#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    
    pub fn square(size: u32) -> Self {
        Self::new(size, size)
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, r2: &Rectangle) -> bool {
        &self.width > &r2.width && &self.height > &r2.height
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        let rect = Rectangle::square(10);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 10);
    }

    #[test]
    fn test_area() {
        let rect = Rectangle::new(20, 10);
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_can_hold_true() {
        let rect1 = Rectangle::new(20, 10);
        let rect2 = Rectangle::new(10, 5);
        assert_eq!(rect1.can_hold(&rect2), true);
    }

    #[test]
    fn test_can_hold_false_inferior_width() {
        let rect1 = Rectangle::new(20, 10);
        let rect2 = Rectangle::new(21, 2);
        assert_eq!(rect1.can_hold(&rect2), false);
    }

    #[test]
    fn test_can_hold_false_inferior_height() {
        let rect1 = Rectangle::new(20, 10);
        let rect2 = Rectangle::new(3, 11);
        assert_eq!(rect1.can_hold(&rect2), false);
    }
    
}