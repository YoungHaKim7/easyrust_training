enum GiveName {
    One(One),
    Two(Two)
}

impl GiveName {
    fn give_name(&self) -> &'static str {
        match self {
            GiveName::One(o) => o.give_name(),
            GiveName::Two(t) => t.give_name(),
        }
    }

}


// fn give_name(&self) -> &'static str;

struct One;

struct Two;

impl One {
    fn give_name(&self) -> &'static str {
        "One"
    }
    fn new() -> GiveName {
        GiveName::One(One)
    }
}

impl Two {
    fn give_name(&self) -> &'static str {
        "Two"
    }
}

fn get_name(input: &GiveName) {
    println!("The type name is : {}", input.give_name());
}


fn main() {
    // let one = GiveName::One(One);
    let one = One::new();
    let two = GiveName::Two(Two);

    get_name(&one);
    get_name(&two);
}


// trait GiveName {
//     fn give_name(&self) -> &'static str;
// }

// struct One;

// struct Two;

// impl GiveName for One {
//     fn give_name(&self) -> &'static str {
//         "One"
//     }
// }

// impl GiveName for Two {
//     fn give_name(&self) -> &'static str {
//         "Two"
//     }
// }

// fn get_name<T: GiveName>(input: &T) {
//     println!("The type name is : {}", input.give_name());
// }

// fn main() {
//     let one = One;
//     let two = Two;
//     get_name(&one);
//     get_name(&two);
// }
