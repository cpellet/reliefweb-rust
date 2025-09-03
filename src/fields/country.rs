//! # Country Endpoint Fields
//!
//! This module defines the structures for the "countries" endpoint in the ReliefWeb API.
//! It includes the `CountryFields` struct and a type alias for the `ResourceEndpoint` specialized to countries.

use serde::{Deserialize, Serialize};

use crate::{
    endpoint::ResourceEndpoint,
    fields::common::{DocumentDates, Location},
};

/// Type alias for a `ResourceEndpoint` specialized for countries.
pub type CountriesEndpoint<'c> = ResourceEndpoint<'c, CountryFields>;

/// Represents the fields of a country record returned by the ReliefWeb API.
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryFields {
    /// The unique identifier of the country.
    pub id: Option<i64>,
    /// Full country name.
    pub name: Option<String>,
    /// Status of the country record (if available).
    pub status: Option<String>,
    /// Short name or abbreviation of the country.
    pub shortname: Option<String>,
    /// ISO 3166-1 alpha-3 code for the country.
    pub iso3: Option<String>,
    /// URL pointing to more information about the country.
    pub url: Option<String>,
    /// Alternative URL or alias.
    pub url_alias: Option<String>,
    /// Various dates associated with the country record (created, changed, etc.).
    pub date: Option<DocumentDates>,
    /// Geographical location (latitude and longitude) of the country.
    pub location: Option<Location>,
}
