use serde::{Deserialize, Serialize};

/// Represents a paginated API response from the ReliefWeb API.
///
/// `T` is the type of the `fields` contained in each item of the `data` array.
///
/// # Example
///
/// ```no_run
/// use reliefweb::response::{ApiResponse, ApiItem};
/// use serde_json::Value;
///
/// let json = r#"
/// {
///     "data": [
///         { "id": "1", "score": 100, "fields": { "title": "Report 1" } }
///     ],
///     "count": 1,
///     "totalCount": 10
/// }
/// "#;
///
/// let resp: ApiResponse<Value> = serde_json::from_str(json).unwrap();
/// assert_eq!(resp.data[0].fields["title"], "Report 1");
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// The URL of this resource.
    pub href: Option<String>,
    /// Timestamp of the API response.
    pub time: Option<u32>,
    /// Navigation links for pagination.
    pub links: Option<Links>,
    /// Total number of items across all pages.
    #[serde(rename = "totalCount")]
    pub total_count: Option<u32>,
    /// Number of items in this response.
    pub count: Option<u32>,
    /// The list of items returned by the API.
    pub data: Vec<ApiItem<T>>,
}

/// Represents pagination and related links for an API response.
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    /// Link to the current page of results.
    #[serde(rename = "self")]
    pub self_: Option<HrefLink>,
    /// Link to the next page of results, if available.
    pub next: Option<HrefLink>,
    /// Link to the previous page of results, if available.
    pub prev: Option<HrefLink>,
}

/// Represents a URL link returned by the API.
#[derive(Debug, Serialize, Deserialize)]
pub struct HrefLink {
    /// The full URL of the link.
    pub href: String,
}

/// Represents an individual item in the API response.
///
/// `T` is the type of the `fields` returned by this resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiItem<T> {
    /// The unique identifier for this item.
    pub id: String,
    /// The relevance or score of this item (may be `None`).
    pub score: Option<u32>,
    /// The main fields of the item.
    pub fields: T,
    /// Optional URL to this item’s API resource.
    pub href: Option<String>,
}
