//! # Report Endpoint Fields
//!
//! This module defines the structures for the "reports" endpoint in the ReliefWeb API.
//! It includes the `ReportFields` struct and a type alias for the `ResourceEndpoint` specialized to reports.

use serde::{Deserialize, Serialize};

use crate::{
    endpoint::ResourceEndpoint,
    fields::common::{Country, Descriptor, DocumentDates, Language, Source},
};

/// Type alias for a `ResourceEndpoint` specialized for reports.
pub type ReportsEndpoint<'c> = ResourceEndpoint<'c, ReportFields>;

/// Represents the fields of a report record returned by the ReliefWeb API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReportFields {
    /// Unique identifier of the report.
    pub id: Option<i64>,
    /// Title of the report.
    pub title: Option<String>,
    /// Status of the report record.
    pub status: Option<String>,
    /// Report description or body.
    pub body: Option<String>,
    /// Origin of the report.
    pub origin: Option<String>,
    /// Primary country associated with the report.
    pub primary_country: Option<Country>,
    /// List of countries associated with the report.
    pub country: Option<Vec<Country>>,
    /// Sources of the report.
    pub source: Option<Vec<Source>>,
    /// Languages in which the report is available.
    pub language: Option<Vec<Language>>,
    /// Themes associated with the report.
    pub theme: Option<Vec<Descriptor>>,
    /// Formats associated with the report.
    pub format: Option<Vec<Descriptor>>,
    /// URL linking to the report.
    pub url: Option<String>,
    /// Alternative URL or alias for the report.
    pub url_alias: Option<String>,
    /// HTML-formatted body of the report.
    #[serde(rename = "body-html")]
    pub body_html: Option<String>,
    /// Various dates associated with the report (created, changed, closing, etc.).
    pub date: Option<DocumentDates>,
}
