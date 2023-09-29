use serde_json::json;
use crate::model::*;
use crate::PetstoreClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ShowPetByIdRequest<'a> {
    pub(crate) http_client: &'a PetstoreClient,
    pub pet_id: String,
}
impl<'a> ShowPetByIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Pet> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/pets/{pet_id}", pet_id = self.pet_id));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for ShowPetByIdRequest<'a> {
    type Output = httpclient::InMemoryResult<Pet>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}