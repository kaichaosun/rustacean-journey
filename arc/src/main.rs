use std::{thread::spawn, sync::Arc};

struct User {
    name: String,
}

fn main() {
    let user_original = Arc::new(User {
        name: "kaichao".to_string(),
    });

    let user = user_original.clone();
    let t1 = spawn(move || {
        println!("Hello from the first thread {}", user.name);
    });

    let user_t2 = user_original.clone();
    let t2 = spawn(move || {
        println!("Hello from the second thread {}", user_t2.name);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
