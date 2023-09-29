use serde_json::json;
use crate::model::*;
use crate::PetstoreClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListPetsRequest<'a> {
    pub(crate) http_client: &'a PetstoreClient,
    pub limit: Option<i64>,
}
impl<'a> ListPetsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Pets> {
        let mut r = self.http_client.client.get("/pets");
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListPetsRequest<'a> {
    type Output = httpclient::InMemoryResult<Pets>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}