struct Test<'a> {
    first: &'a str,
}

fn main() {
    let y = "Hello";
    let f = Test { first: y };

    println!("{}", f.first);
}