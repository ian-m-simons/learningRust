fn main() {
    let mut number = 0;
    let result = loop {
        number += 1;
        if number == 10 {
            break number * 2;
        }
    };
    println!("The result is {result}")
}
