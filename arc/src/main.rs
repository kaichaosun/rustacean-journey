use std::thread::spawn;

struct User {
    name: String,
}

fn main() {
    let user = User {
        name: "kaichao".to_string(),
    };

    spawn(move || {
        println!("Hello from the first thread {}", user.name);
    }).join().unwrap();
}
