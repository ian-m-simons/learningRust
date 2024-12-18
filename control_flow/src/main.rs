fn main() {
    let mut number: i32 = 0;
    loop {
        let value = fib(number);
        println!("{value}");
        number += 1
    }
}

fn fib(num: i32) -> i32 {
    let mut value: i32 = 0;
    if num == 0 {
        return 0;
    }
    else if num == 1 {
        return 1;
    }
    else {
        value = fib(num-1) + fib (num-2);
        return value;
    }
}
