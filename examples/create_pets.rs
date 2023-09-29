#![allow(unused_imports)]
use petstore::PetstoreClient;
use petstore::model::*;
#[tokio::main]
async fn main() {
    let client = PetstoreClient::from_env();
    let response = client.create_pets().await.unwrap();
    println!("{:#?}", response);
}