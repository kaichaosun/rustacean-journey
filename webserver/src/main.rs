use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

fn main() {
    println!("Hello, server!");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
