//! [`PetstoreClient`](struct.PetstoreClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub struct PetstoreClient {
    pub client: httpclient::Client,
}
impl PetstoreClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("http://petstore.swagger.io/v1"),
        }
    }
}
impl PetstoreClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client }
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///List all pets
    pub fn list_pets(&self) -> request::ListPetsRequest {
        request::ListPetsRequest {
            http_client: &self,
            limit: None,
        }
    }
    ///Create a pet
    pub fn create_pets(&self) -> request::CreatePetsRequest {
        request::CreatePetsRequest {
            http_client: &self,
        }
    }
    ///Info for a specific pet
    pub fn show_pet_by_id(&self, pet_id: &str) -> request::ShowPetByIdRequest {
        request::ShowPetByIdRequest {
            http_client: &self,
            pet_id: pet_id.to_owned(),
        }
    }
}