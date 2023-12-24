fn main() {
    let mut number_false = false;

    while !number_false {
        // should be while number_false {, not !number_false}
        let my_number = 3;
        if my_number < 5 {
            println!("condition was true");
            number_false = true;
        } else {
            println!("condition was false");
            number_false = false; // should set number_false to false again in this branch, not true
        }
    }
}
