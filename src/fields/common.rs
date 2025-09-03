//! # Common Fields
//!
//! This module defines shared structures used across multiple ReliefWeb endpoints,
//! such as countries, locations, document dates, descriptors, languages, and sources.
//!
//! These types are typically embedded within endpoint-specific structs (e.g., `ReportFields`, `BlogFields`).

use serde::{Deserialize, Serialize};

/// Represents a country associated with a record.
#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    /// Link to the API resource for this country.
    pub href: Option<String>,
    /// The unique identifier of the country.
    pub id: Option<i64>,
    /// Full country name.
    pub name: Option<String>,
    /// Short name (abbreviation) of the country.
    pub shortname: Option<String>,
    /// ISO 3166-1 alpha-3 code.
    pub iso3: Option<String>,
    /// Geographical location (latitude and longitude).
    pub location: Option<Location>,
    /// Indicates if this is the primary country for the record.
    pub primary: Option<bool>,
}

/// Represents a geographical location with latitude and longitude.
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    /// Latitude coordinate.
    pub lat: Option<f64>,
    /// Longitude coordinate.
    pub lon: Option<f64>,
}

/// Represents various dates associated with a document or record.
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDates {
    /// Closing date of the document (if applicable).
    pub closing: Option<String>,
    /// Original date of the document.
    pub original: Option<String>,
    /// Date when the document was last changed.
    pub changed: Option<String>,
    /// Date when the document was created.
    pub created: Option<String>,
}

/// Represents a generic descriptor, used for types like source types.
#[derive(Debug, Serialize, Deserialize)]
pub struct Descriptor {
    /// The unique identifier of the descriptor.
    pub id: Option<i64>,
    /// Name of the descriptor.
    pub name: Option<String>,
}

/// Represents a language associated with a record.
#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    /// The unique identifier of the language.
    pub id: Option<i64>,
    /// The language name.
    pub name: Option<String>,
    /// ISO or internal code for the language.
    pub code: Option<String>,
}

/// Represents a source (organization or entity) related to a record.
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    /// Link to the API resource for this source.
    pub href: Option<String>,
    /// The unique identifier of the source.
    pub id: Option<i64>,
    /// The short name of the source.
    pub name: Option<String>,
    /// Optional shortname alternative.
    pub shortname: Option<String>,
    /// Long name of the source.
    pub longname: Option<String>,
    /// Spanish name (if applicable).
    pub spanish_name: Option<String>,
    /// Homepage URL of the source.
    pub homepage: Option<String>,
    /// Type of source (e.g., NGO, government, media).
    #[serde(rename = "type")]
    pub source_type: Option<Descriptor>,
}
