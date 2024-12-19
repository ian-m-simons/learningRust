struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.length
    }
}
fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    println!("area = {}", rect1.area());
}
