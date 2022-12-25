// destructuring

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}


fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false
    };

    println!("They call hime {} but his real name is {}. He is {} cm tall and is he happy? {}",
            papa_doc.name, papa_doc.real_name, papa_doc.height, papa_doc.happiness);
}
