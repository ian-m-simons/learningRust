enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m = Message::Write(String::from("hello"));
    let some_number = some(5);
    let some_char = some('e');

    let absent_number: Option<i32> = none;

}
