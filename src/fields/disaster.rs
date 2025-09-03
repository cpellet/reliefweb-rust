//! # Disaster Endpoint Fields
//!
//! This module defines the structures for the "disasters" endpoint in the ReliefWeb API.
//! It includes the `DisasterFields` struct and a type alias for the `ResourceEndpoint` specialized to disasters.

use serde::{Deserialize, Serialize};

use crate::{
    endpoint::ResourceEndpoint,
    fields::common::{Country, DocumentDates},
};

/// Type alias for a `ResourceEndpoint` specialized for disasters.
pub type DisastersEndpoint<'c> = ResourceEndpoint<'c, DisasterFields>;

/// Represents the fields of a disaster record returned by the ReliefWeb API.
#[derive(Debug, Serialize, Deserialize)]
pub struct DisasterFields {
    /// Unique identifier of the disaster.
    pub id: Option<i64>,
    /// Name of the disaster.
    pub name: Option<String>,
    /// Short description of the disaster.
    pub description: Option<String>,
    /// Status of the disaster record.
    pub status: Option<String>,
    /// GLIDE identifier for the disaster.
    pub glide: Option<String>,
    /// The primary country affected by the disaster.
    pub primary_country: Option<Country>,
    /// The primary type of disaster.
    pub primary_type: Option<Type>,
    /// List of countries affected by the disaster.
    pub country: Option<Vec<Country>>,
    /// List of disaster types.
    #[serde(rename = "type")]
    pub disaster_fields_type: Option<Vec<Type>>,
    /// URL pointing to more information about the disaster.
    pub url: Option<String>,
    /// Alternative URL or alias.
    pub url_alias: Option<String>,
    /// Various dates associated with the disaster record (created, changed, closing, etc.).
    pub date: Option<DocumentDates>,
    /// HTML-formatted description of the disaster.
    #[serde(rename = "description-html")]
    pub description_html: Option<String>,
    /// Optional profile information (overview) of the disaster.
    pub profile: Option<Profile>,
}

/// Represents a disaster type.
#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    /// Unique identifier of the type.
    pub id: Option<i64>,
    /// Name of the type.
    pub name: Option<String>,
    /// Optional code associated with the type.
    pub code: Option<String>,
    /// Whether this type is marked as primary.
    pub primary: Option<bool>,
}

/// Represents an optional profile for a disaster, including HTML overview.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    /// Plain text overview of the disaster.
    pub overview: Option<String>,
    /// HTML-formatted overview.
    pub overview_html: Option<String>,
}
