//! # Source Endpoint Fields
//!
//! This module defines the structures for the "sources" endpoint in the ReliefWeb API.
//! It includes the `SourceFields` struct and a type alias for the `ResourceEndpoint` specialized to sources.

use serde::{Deserialize, Serialize};

use crate::{
    endpoint::ResourceEndpoint,
    fields::common::{Country, Descriptor, DocumentDates},
};

/// Type alias for a `ResourceEndpoint` specialized for sources.
pub type SourcesEndpoint<'c> = ResourceEndpoint<'c, SourceFields>;

/// Represents the fields of a source record returned by the ReliefWeb API.
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFields {
    /// Unique identifier of the source.
    pub id: Option<i64>,
    /// Name of the source.
    pub name: Option<String>,
    /// Status of the source record.
    pub status: Option<String>,
    /// Short name or abbreviation of the source.
    pub shortname: Option<String>,
    /// Content types associated with the source.
    pub content_type: Option<Vec<String>>,
    /// Type of the source.
    #[serde(rename = "type")]
    pub source_fields_type: Option<Descriptor>,
    /// Countries associated with the source.
    pub country: Option<Vec<Country>>,
    /// URL linking to the source.
    pub url: Option<String>,
    /// Alternative URL or alias for the source.
    pub url_alias: Option<String>,
    /// Various dates associated with the source (created, changed, etc.).
    pub date: Option<DocumentDates>,
}
