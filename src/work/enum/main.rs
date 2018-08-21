#[derive(Debug)]
enum DogKind {
    Azawakh(String),
    Whippet,
}

struct Dog {
    kind: DogKind,
    name: String,
}

fn main() {
    let my_dog = Dog {
        kind: DogKind::Azawakh(String::from("Edora")),
        name: String::from("Edora"),
    };

    println!("My first dog is {} of kind {:?}", my_dog.name, my_dog.kind);

    let my_dog2 = Dog {
        kind: DogKind::Whippet,
        name: String::from("Djouba"),
    };

    println!("My second dog is {} of kind {:?}", my_dog2.name, my_dog2.kind);
}