use std::sync::Mutex;

#[derive(Debug)]
struct Log {
    date: &'static str,
    message: String
}

static GLOBAL_LOGGER: Mutex<Vec<Log>> = Mutex::new(Vec::new());

fn do_thing(date: &'static str) {
    GLOBAL_LOGGER.lock().unwrap().push(Log {
        date,
        message: "Everything's fine".to_string()
    });
}

fn main() {
    do_thing("2022-09-12");
    do_thing("2023-05-05");
    println!("{GLOBAL_LOGGER:#?}");
}
