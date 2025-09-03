use anyhow::Result;
use serde::de::DeserializeOwned;

use crate::{
    Client,
    params::{QueryParams, QueryProfile},
    response::ApiResponse,
};

/// Generic endpoint wrapper for any ReliefWeb resource.
///
/// `T` is the type of the `fields` returned by the API for this resource.
/// Use concrete types like [`ReportFields`] when possible, or `serde_json::Value` for generic use.
///
/// # Example
///
/// ```no_run
/// use reliefweb::{Client, ResourceEndpoint, QueryParams};
/// use serde_json::Value;
///
/// let client = Client::new("api.reliefweb.int", "my_app", reliefweb::APIVersion::V2).unwrap();
/// let reports_endpoint: ResourceEndpoint<Value> = ResourceEndpoint::new(&client, "reports");
///
/// let list = reports_endpoint.list(Some(&QueryParams::new().limit(5))).await.unwrap();
/// ```
pub struct ResourceEndpoint<'c, T> {
    client: &'c Client,
    resource: &'static str,
    _marker: std::marker::PhantomData<T>,
}

impl<'c, T> ResourceEndpoint<'c, T>
where
    T: DeserializeOwned,
{
    /// Creates a new endpoint instance from the given `Client`, accessible at `{base_endpoint}/resource`
    pub fn new(client: &'c Client, resource: &'static str) -> Self {
        Self {
            client,
            resource,
            _marker: std::marker::PhantomData,
        }
    }

    /// Execute a `list` request to the endpoint.
    ///
    /// Use `options` to specify all supported query options for the request.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<ApiResponse<T>> {
        let endpoint = self.client.api_base.join(self.resource)?;
        let resp = self
            .client
            .get_with_params(endpoint, params)
            .send()
            .await?
            .json::<ApiResponse<T>>()
            .await?;
        Ok(resp)
    }

    /// Execute a `get` request for a specific resource `id` on the endpoint.
    ///
    /// Use `profile` to set the resoure data profile returned by the request.
    /// Use `include` and `exclude` to specify the exact fields that should be returned by the API.
    pub async fn get(
        &self,
        id: &str,
        profile: Option<QueryProfile>,
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
    ) -> Result<ApiResponse<T>> {
        let endpoint = self
            .client
            .api_base
            .join(&format!("{}/{}", self.resource, id))?;
        let mut params = QueryParams::new();
        if let Some(p) = profile {
            params = params.profile(p);
        }
        if let Some(i) = include {
            params = params.include(i);
        }
        if let Some(e) = exclude {
            params = params.exclude(e);
        }

        let resp = self
            .client
            .get_with_params(endpoint, Some(&params))
            .send()
            .await?
            .json::<ApiResponse<T>>()
            .await?;
        Ok(resp)
    }
}

impl<'c, T> ResourceEndpoint<'c, T> {
    #[cfg(test)]
    pub fn resource(&self) -> &'static str {
        self.resource
    }
}

#[cfg(test)]
mod tests {

    use crate::Client;

    #[tokio::test]
    async fn test_list_reports() {
        use httpmock::prelude::*;

        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/reports")
                .query_param("appname", "testapp");
            then.status(200).json_body_obj(&serde_json::json!({
                "data": [
                    { "id": "1", "score" : 1, "fields": { "title": "Report 1" } }
                ]
            }));
        });

        let client = Client::new_with_scheme(
            "http",
            format!("{}:{}", server.host(), server.port()).as_str(),
            "testapp",
            crate::APIVersion::V2,
        )
        .unwrap();

        let resp = client.reports().list(None).await.unwrap();

        assert_eq!(resp.data[0].fields.title, Some("Report 1".to_string()));

        mock.assert();
    }

    #[tokio::test]
    async fn test_get_report() {
        use httpmock::prelude::*;

        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/reports/123")
                .query_param("appname", "testapp");
            then.status(200).json_body_obj(&serde_json::json!({
                "data": [
                    { "id": "123",  "score" : 1, "fields": { "title": "Report 123" } }
                    ]
            }));
        });

        let client = Client::new_with_scheme(
            "http",
            format!("{}:{}", server.host(), server.port()).as_str(),
            "testapp",
            crate::APIVersion::V2,
        )
        .unwrap();

        let resp = client.reports().get("123", None, None, None).await.unwrap();

        assert_eq!(resp.data[0].fields.title, Some("Report 123".to_string()));
        mock.assert();
    }
}
