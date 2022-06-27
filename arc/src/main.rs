// resource: https://itsallaboutthebit.com/arc-mutex/#

use std::{thread::{spawn, sleep}, sync::{Arc, Mutex}, time::Duration};

struct User {
    name: String,
}

fn main() {
    let user_original = Arc::new(Mutex::new(User {
        name: "kaichao".to_string(),
    }));

    let user = user_original.clone();
    let t1 = spawn(move || {
        let mut locked_user = user.lock().unwrap();
        println!("Hello from the first thread {}", locked_user.name);
        locked_user.name = String::from("jack");
        println!("Hello from the first thread {}", locked_user.name);
    });

    let user_t2 = user_original.clone();
    let t2 = spawn(move || {
        sleep(Duration::from_millis(100));
        println!("Hello from the second thread {}", user_t2.lock().unwrap().name);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
