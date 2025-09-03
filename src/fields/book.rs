//! # Book Endpoint Fields
//!
//! This module defines the structures for the "book" endpoint in the ReliefWeb API.
//! It includes the `BookFields` struct and a type alias for the `ResourceEndpoint` specialized to books.

use crate::{common::DocumentDates, endpoint::ResourceEndpoint};
use serde::{Deserialize, Serialize};

/// Type alias for a `ResourceEndpoint` specialized for books.
pub type BooksEndpoint<'c> = ResourceEndpoint<'c, BookFields>;

/// Represents the fields of a book record returned by the ReliefWeb API.
#[derive(Debug, Serialize, Deserialize)]
pub struct BookFields {
    /// Unique identifier of the book.
    pub id: Option<i64>,
    /// Title of the book.
    pub title: Option<String>,
    /// Status of the book record.
    pub status: Option<String>,
    /// Main body content of the book.
    pub body: Option<String>,
    /// URL linking to the book record.
    pub url: Option<String>,
    /// Alternative URL or alias for the book.
    pub url_alias: Option<String>,
    /// HTML body content of the book.
    #[serde(rename = "body-html")]
    pub body_html: Option<String>,
    /// Various dates associated with the book (created, changed, etc.).
    pub date: Option<DocumentDates>,
}
