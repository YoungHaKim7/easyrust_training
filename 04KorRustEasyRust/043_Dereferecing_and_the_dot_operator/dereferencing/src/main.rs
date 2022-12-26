struct Item {
    number: u8,
}

// . dot operator
impl Item {
    fn compare_number(&self, other_number: u8) {
        println!("Are they equal? {}", self.number == other_number)
    }
}

fn main() {
    let item = Item { number: 10 };

    let reference_item = &item;

    item.compare_number(10);
}
