use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/blocks")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body()
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
