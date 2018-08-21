
fn main() {
    let list: Vec<i32> = vec!(1, 2, 3, 4, 5, 666);
    println!("List {:?}", list);

    for val in list {
        match val {
            1 => println!("Matched number {}", val),
            2 => println!("Matched number {}", val),
            3 => println!("Matched number {}", val),
            4 => println!("Matched number {}", val),
            5 => println!("Matched number {}", val),
            _ => println!("Matched default for number {}", val),
        };
    }
}