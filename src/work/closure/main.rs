use std::thread;
use std::time::Duration;


fn main() {
    let number: i32 = 30;
    println!("Number {}", number);

    let number_closure = |numb: i32| {
        println!("Calculating next number...");
        thread::sleep(Duration::from_secs(1));
        numb + 1
    };
    println!("Number {}", number_closure(number));
}