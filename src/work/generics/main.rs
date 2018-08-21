#[derive(Copy, Clone)]
struct Test<Val> {
    first: Val,
}
trait GenericTest<Val> {
    fn gimme_value(self) -> Val;
}

impl<Val> GenericTest<Val> for Test<Val> {

    fn gimme_value(self) -> Val {
        self.first
    }
}

fn find_biggest<U: Copy + GenericTest<G>, G:  PartialOrd>(list : &[U]) -> G {
    let mut largest = list[0].gimme_value();

    for &item in list.iter() {
        if item.gimme_value() > largest {
            largest = item.gimme_value();
        }
    }

    largest
}

fn main() {
    let number_list = vec![Test{first: 34}, Test{first: 50}, Test{first: 25}, Test{first: 100}, Test{first: 65}];

    let result = find_biggest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![Test{first: "bgebe"}, Test{first: "toto"}, Test{first: "tata"}, Test{first: "zfrezg"}, Test{first: "qzgrezbre"}];

    let result = find_biggest(&number_list);
    println!("The 'largest' text is {}", result);
}