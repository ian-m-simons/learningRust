/*this is a block comment
 * I just needed to clarify that this works
 * thank you for your time
 */

fn main() {
    let q = another_function(5,3);//setting value of q
    
    println!("The Value of q is {q}"); //printing q for the user
}

fn another_function(x: i32, y: i32) -> i32 {
    x+y //calculating value to return
}
