#![allow(unused_imports)]
use petstore::PetstoreClient;
use petstore::model::*;
#[tokio::main]
async fn main() {
    let client = PetstoreClient::from_env();
    let pet_id = "your pet id";
    let response = client.show_pet_by_id(pet_id).await.unwrap();
    println!("{:#?}", response);
}