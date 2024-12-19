struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    println!("area = {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 { 
    rectangle.length * rectangle.width
}
