use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fmt;
use serde::{Serialize, Deserialize};

fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

fn posts() -> impl Responder {
    let post = Post {
        title: String::from("Start Rust"),
        author: String::from("Kaichao"),
        content: String::from("The easy way to learn Rust")
    };
    HttpResponse::Ok().body(serde_json::to_string(&post).unwrap())
}

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    title: String,
    author: String,
    content: String
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "title: {}, author: {}, content: {}", self.title, self.author, self.content)
    }
}

fn main() {
    println!("Hello, server!");

    let post = Post {
        title: String::from("mybook"),
        author: String::from("kaichao"),
        content: String::from("hello rust")
    };
    // Convert the Post to a JSON string.
    let serialized = serde_json::to_string(&post).unwrap();

    println!("serialized = {}", serialized);

    // Convert JSON string to Post
    let deserialized: Post = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/posts", web::get().to(posts))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
