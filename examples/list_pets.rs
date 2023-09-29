#![allow(unused_imports)]
use petstore::PetstoreClient;
use petstore::model::*;
#[tokio::main]
async fn main() {
    let client = PetstoreClient::from_env();
    let response = client.list_pets().limit(1).await.unwrap();
    println!("{:#?}", response);
}