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
