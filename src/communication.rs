use actix_web::{HttpResponse, Responder};

use crate::block::Block;

async fn get_blockchain(blockchain: &Vec<Block>) -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
