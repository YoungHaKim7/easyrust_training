// destructuring

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

#[derive(Debug)]
struct Person2 {
    name: String,
    height: u8,
}

impl Person2 {
    fn from_person(i: Person) -> Self {
        let Person { name, height, .. } = i;

        Self { name, height }
    }
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    let person2 = Person2::from_person(papa_doc);

    println!("Person2 type is : {:?}", person2);
}
