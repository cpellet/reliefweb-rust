//! # Blog Endpoint Fields
//!
//! This module defines the structures for the "blog" endpoint in the ReliefWeb API.
//! It includes the `BlogFields` struct and a type alias for the `ResourceEndpoint` specialized to blogs.

use crate::{endpoint::ResourceEndpoint, fields::common::DocumentDates};
use serde::{Deserialize, Serialize};

/// Type alias for a `ResourceEndpoint` specialized for blogs.
pub type BlogsEndpoint<'c> = ResourceEndpoint<'c, BlogFields>;

/// Represents the fields of a blog record returned by the ReliefWeb API.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogFields {
    /// Unique identifier of the blog.
    pub id: Option<i64>,
    /// Title of the blog.
    pub title: Option<String>,
    /// Status of the blog record.
    pub status: Option<String>,
    /// Main body content of the blog.
    pub body: Option<String>,
    /// Author of the blog.
    pub author: Option<String>,
    /// URL linking to the blog record.
    pub url: Option<String>,
    /// Alternative URL or alias for the blog.
    pub url_alias: Option<String>,
    /// HTML body content of the blog.
    #[serde(rename = "body-html")]
    pub body_html: Option<String>,
    /// Various dates associated with the blog (created, changed, etc.).
    pub date: Option<DocumentDates>,
}
