fn main() {
    let q = another_function(5,3);

    println!("The Value of q is {q}"); 
}

fn another_function(x: i32, y: i32) -> i32 {
    x+y
}
