use std::sync::Mutex;

use actix_web::{
    get, post,
    web::{self, block},
    HttpResponse, Responder,
};

use crate::{
    block::Block,
    types::{Blockchain, SharedState},
};

#[post("/mine")]
async fn mine(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/bloks")]
async fn get_blockchain(blockchain: web::Data<SharedState>) -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&(*blockchain.lock().unwrap())).unwrap())
}
