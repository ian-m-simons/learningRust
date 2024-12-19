struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.length
    }
    fn perimeter(&self) -> u32 {
        (self.width * 2) + (self.length * 2)
    }
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        if self.length > rect2.length() && self.width > rect2.width(){
            return true;
        }
        false
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            length: size,
        }
    }
    //getters
    fn length(&self) -> u32 {
        self.length
    }
    fn width(&self) -> u32 {
        self.width
    }
}
fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle { 
        length: 20,
        width: 10,
    };

    let sqr1 = Rectangle::square(5);
    println!("can rectangle 1 hold rectangle 2?\n correct answer is {}", rect1.can_hold(&rect2));
    println!("area of square 1 is {}", sqr1.area());

}
