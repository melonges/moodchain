use actix_web::{post, web::block, HttpResponse, Responder};

use crate::block::Block;

#[post("/mine")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn get_blockchain(blockchain: &Vec<Block>) -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(blockchain).unwrap())
}
