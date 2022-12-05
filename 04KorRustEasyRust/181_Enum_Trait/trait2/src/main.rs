trait GiveName {
    fn give_name(&self) -> &'static str;
}

struct One;

struct Two;

impl GiveName for One {
    fn give_name(&self) -> &'static str {
        "One"
    }
}

impl GiveName for Two {
    fn give_name(&self) -> &'static str {
        "Two"
    }
}

fn get_name<T: GiveName>(input: &T) {
    println!("The type name is : {}", input.give_name());
}

fn main() {
    let one = One;
    let two = Two;
    get_name(&one);
    get_name(&two);
}
