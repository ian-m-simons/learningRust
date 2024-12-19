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
}
fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    println!("area = {}\nperimeter = {}", rect1.area(), rect1.perimeter());
}
