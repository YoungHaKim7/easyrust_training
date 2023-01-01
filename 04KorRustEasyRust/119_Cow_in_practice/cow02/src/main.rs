use std::borrow::Cow;

#[derive(Debug)]
struct User<'a> {
    name: Cow<'a, str>,
}

fn main() {
    let name_1 = "User 1";
    let name_2 = "User 2".to_string();

    let user_1 = User {
        name: name_1.into(),
    };

    let user_2 = User {
        name: name_2.into(),
    };

    println!("User 1 is {user_1:?} and User 2 is {user_2:?}");
}
