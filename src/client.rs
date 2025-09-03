use std::fmt;

use anyhow::Result;
use reqwest::Url;

use crate::{
    fields::{
        blog::BlogsEndpoint, book::BooksEndpoint, country::CountriesEndpoint,
        disaster::DisastersEndpoint, job::JobsEndpoint, report::ReportsEndpoint,
        source::SourcesEndpoint, training::TrainingsEndpoint,
    },
    params::QueryParams,
};

/// ReliefWeb API's public instance base URL.
pub const RELIEFWEB_DOMAIN: &str = "api.reliefweb.int";

/// A client for interacting with the ReliefWeb API.
///
/// # Examples
///
/// ```no_run
/// use reliefweb::{Client, APIVersion};
///
/// let client = Client::new("api.reliefweb.int", "my_app", APIVersion::V2).unwrap();
/// let reports_endpoint = client.reports();
pub struct Client {
    /// Base URL for the API.
    pub(crate) api_base: Url,

    /// Underlying HTTP client.
    pub(crate) client: reqwest::Client,

    /// The application name to identify your requests.
    pub(crate) app_name: String,
}

/// The API specification version.
/// V1 is deprecated and should no longer be used.
/// The V2 version is fully compatible with the V1 version.
pub enum APIVersion {
    V1,
    V2,
}

impl fmt::Display for APIVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            APIVersion::V1 => write!(f, "v1"),
            APIVersion::V2 => write!(f, "v2"),
        }
    }
}

impl Client {
    /// Create a new instance of client with the given domain, application name and specification version, using HTTPS transport.
    pub fn new(domain: &str, app_name: &str, version: APIVersion) -> Result<Client> {
        let api_base = Url::parse(format!("https://{domain}/{version}/").as_str())?;
        let client = reqwest::Client::new();
        Ok(Client {
            api_base,
            client,
            app_name: app_name.to_string(),
        })
    }

    /// Create a new instance of client with the given transport scheme, domain, application name and specification version.
    pub fn new_with_scheme(
        scheme: &str,
        domain: &str,
        app_name: &str,
        version: APIVersion,
    ) -> Result<Client> {
        let api_base = Url::parse(&format!("{scheme}://{domain}/{version}/"))?;
        let client = reqwest::Client::new();
        Ok(Client {
            api_base,
            client,
            app_name: app_name.to_string(),
        })
    }

    /// Returns the [`ReportsEndpoint`] to interact with the `reports` API.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use reliefweb::{Client, APIVersion, QueryParams};
    ///
    /// let client = Client::new("api.reliefweb.int", "my_app", APIVersion::V2).unwrap();
    /// let reports = client.reports()
    ///     .list(Some(&QueryParams::new().limit(10)))
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn reports(&self) -> ReportsEndpoint {
        ReportsEndpoint::new(self, "reports")
    }

    /// Returns the [`DisastersEndpoint`] to interact with the `disasters` API.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use reliefweb::{Client, APIVersion, QueryParams};
    ///
    /// let client = Client::new("api.reliefweb.int", "my_app", APIVersion::V2).unwrap();
    /// let disasters = client.disasters()
    ///     .list(Some(&QueryParams::new().limit(10)))
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn disasters(&self) -> DisastersEndpoint {
        DisastersEndpoint::new(self, "disasters")
    }

    /// Returns the [`CountriesEndpoint`] to interact with the `countries` API.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use reliefweb::{Client, APIVersion, QueryParams};
    ///
    /// let client = Client::new("api.reliefweb.int", "my_app", APIVersion::V2).unwrap();
    /// let countries = client.countries()
    ///     .list(Some(&QueryParams::new().limit(10)))
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn countries(&self) -> CountriesEndpoint {
        CountriesEndpoint::new(self, "countries")
    }

    /// Returns the [`JobsEndpoint`] to interact with the `jobs` API.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use reliefweb::{Client, APIVersion, QueryParams};
    ///
    /// let client = Client::new("api.reliefweb.int", "my_app", APIVersion::V2).unwrap();
    /// let jobs = client.jobs()
    ///     .list(Some(&QueryParams::new().limit(10)))
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn jobs(&self) -> JobsEndpoint {
        JobsEndpoint::new(self, "jobs")
    }

    /// Returns the [`TrainingsEndpoint`] to interact with the `trainings` API.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use reliefweb::{Client, APIVersion, QueryParams};
    ///
    /// let client = Client::new("api.reliefweb.int", "my_app", APIVersion::V2).unwrap();
    /// let trainings = client.trainings()
    ///     .list(Some(&QueryParams::new().limit(10)))
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn training(&self) -> TrainingsEndpoint {
        TrainingsEndpoint::new(self, "training")
    }

    /// Returns the [`SourcesEndpoint`] to interact with the `sources` API.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use reliefweb::{Client, APIVersion, QueryParams};
    ///
    /// let client = Client::new("api.reliefweb.int", "my_app", APIVersion::V2).unwrap();
    /// let sources = client.sources()
    ///     .list(Some(&QueryParams::new().limit(10)))
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn sources(&self) -> SourcesEndpoint {
        SourcesEndpoint::new(self, "sources")
    }

    /// Returns the [`BlogsEndpoint`] to interact with the `blogs` API.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use reliefweb::{Client, APIVersion, QueryParams};
    ///
    /// let client = Client::new("api.reliefweb.int", "my_app", APIVersion::V2).unwrap();
    /// let posts = client.blogs()
    ///     .list(Some(&QueryParams::new().limit(10)))
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn blog(&self) -> BlogsEndpoint {
        BlogsEndpoint::new(self, "blog")
    }

    /// Returns the [`BooksEndpoint`] to interact with the `books` API.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use reliefweb::{Client, APIVersion, QueryParams};
    ///
    /// let client = Client::new("api.reliefweb.int", "my_app", APIVersion::V2).unwrap();
    /// let books = client.books()
    ///     .list(Some(&QueryParams::new().limit(10)))
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn book(&self) -> BooksEndpoint {
        BooksEndpoint::new(self, "book")
    }

    /// Constructs a GET request to the API with the given endpoint and params.
    /// Includes the `app_name` specified on Client creation as a query parameter.
    pub(crate) fn get_with_params(
        &self,
        mut endpoint: Url,
        params: Option<&QueryParams>,
    ) -> reqwest::RequestBuilder {
        endpoint
            .query_pairs_mut()
            .append_pair("appname", &self.app_name);
        if let Some(p) = params {
            p.apply_to_url(&mut endpoint);
        }
        self.client.get(endpoint)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::params::QueryQuery;

    use super::*;

    #[test]
    fn client_init() {
        let app_name = "reliefweb_rust_tests";
        let c1 = Client::new(RELIEFWEB_DOMAIN, app_name, APIVersion::V1).unwrap();
        assert_eq!(
            c1.api_base.as_str(),
            format!("https://{RELIEFWEB_DOMAIN}/v1/")
        );
        let c2 = Client::new(RELIEFWEB_DOMAIN, app_name, APIVersion::V2).unwrap();
        assert_eq!(
            c2.api_base.as_str(),
            format!("https://{RELIEFWEB_DOMAIN}/v2/")
        );
        assert_eq!(c2.app_name, app_name);
        let result = Client::new("not a url", "app", APIVersion::V2);
        assert!(result.is_err());
    }

    #[test]
    fn get_with_params_none() {
        let client = Client::new(RELIEFWEB_DOMAIN, "app", APIVersion::V2).unwrap();
        let request = client
            .get_with_params(
                Url::parse(&format!("{}/reports", client.api_base)).unwrap(),
                None,
            )
            .build()
            .unwrap();

        let mut pairs = request.url().query_pairs();
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("appname"), Cow::Borrowed("app")))
        );
        assert_eq!(pairs.next(), None);
    }

    #[test]
    fn get_with_params() {
        let client = Client::new(RELIEFWEB_DOMAIN, "reliefweb_rust_tests", APIVersion::V2).unwrap();
        let query = QueryQuery {
            value: "bar".to_string(),
            fields: vec!["foo".to_string()],
            operator: Some(crate::params::FilterOperator::AND),
        };
        let params = QueryParams::new().query(query);
        let request = client
            .get_with_params(
                Url::parse(format!("{}reports", client.api_base).as_str()).unwrap(),
                Some(&params),
            )
            .build()
            .unwrap();
        let mut pairs = request.url().query_pairs();
        assert_eq!(
            pairs.next(),
            Some((
                Cow::Borrowed("appname"),
                Cow::Borrowed("reliefweb_rust_tests")
            ))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("query[0][value]"), Cow::Borrowed("bar")))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("query[0][fields][0]"), Cow::Borrowed("foo")))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("query[0][operator]"), Cow::Borrowed("AND")))
        );
        assert_eq!(pairs.next(), None);
    }

    #[test]
    fn get_with_params_encoding() {
        let client = Client::new(RELIEFWEB_DOMAIN, "app", APIVersion::V2).unwrap();
        let query = QueryQuery {
            value: "foo bar".to_string(),
            fields: vec!["field+name".to_string()],
            operator: None,
        };
        let params = QueryParams::new().query(query);
        let request = client
            .get_with_params(
                Url::parse(&format!("{}reports", client.api_base)).unwrap(),
                Some(&params),
            )
            .build()
            .unwrap();

        let url = request.url().as_str();
        assert!(url.contains("foo+bar")); // spaces encoded
        assert!(url.contains("field%2Bname")); // plus encoded
    }

    #[test]
    fn client_endpoints() {
        let client = Client::new(RELIEFWEB_DOMAIN, "app", APIVersion::V2).unwrap();

        let reports = client.reports();
        let disasters = client.disasters();
        let countries = client.countries();
        let jobs = client.jobs();
        let training = client.training();
        let sources = client.sources();
        let blog = client.blog();
        let book = client.book();

        assert_eq!(reports.resource(), "reports");
        assert_eq!(disasters.resource(), "disasters");
        assert_eq!(countries.resource(), "countries");
        assert_eq!(jobs.resource(), "jobs");
        assert_eq!(training.resource(), "training");
        assert_eq!(sources.resource(), "sources");
        assert_eq!(blog.resource(), "blog");
        assert_eq!(book.resource(), "book");
    }
}
