use std::collections::HashMap;

fn main() {
    let mut mails = HashMap::new();

    mails.insert(String::from("a"), String::from("a.com"));
    mails.insert(String::from("b"), String::from("b.com"));
    mails.insert(String::from("c"), String::from("c.com"));

    for (key, value) in &mails {
        println!("{}: {}", key, value);
    }
}
