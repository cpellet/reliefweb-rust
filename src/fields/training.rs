//! # Training Endpoint Fields
//!
//! This module defines the structures for the "training" endpoint in the ReliefWeb API.
//! It includes the `TrainingFields` struct and a type alias for the `ResourceEndpoint` specialized to trainings.

use serde::{Deserialize, Serialize};

use crate::{
    endpoint::ResourceEndpoint,
    fields::common::{Descriptor, DocumentDates, Language},
};

/// Type alias for a `ResourceEndpoint` specialized for trainings.
pub type TrainingsEndpoint<'c> = ResourceEndpoint<'c, TrainingFields>;

/// Represents the fields of a training record returned by the ReliefWeb API.
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingFields {
    /// Unique identifier of the training.
    pub id: Option<i64>,
    /// Title of the training.
    pub title: Option<String>,
    /// Status of the training record.
    pub status: Option<String>,
    /// Cost of attending the training.
    pub cost: Option<String>,
    /// Main body content of the training record.
    pub body: Option<String>,
    /// URL of the associated event.
    pub event_url: Option<String>,
    /// Instructions on how to register for the training.
    pub how_to_register: Option<String>,
    /// Sources associated with the training.
    pub source: Option<Vec<Source>>,
    /// Languages available for the training.
    pub language: Option<Vec<Language>>,
    /// Themes associated with the training.
    pub theme: Option<Vec<Descriptor>>,
    /// Types of the training.
    #[serde(rename = "type")]
    pub training_fields_type: Option<Vec<Descriptor>>,
    /// Formats available for the training.
    pub format: Option<Vec<Descriptor>>,
    /// Additional languages of the training.
    pub training_language: Option<Vec<Language>>,
    /// URL linking to the training record.
    pub url: Option<String>,
    /// Alternative URL or alias for the training.
    pub url_alias: Option<String>,
    /// HTML body content of the training.
    #[serde(rename = "body-html")]
    pub body_html: Option<String>,
    /// Various dates associated with the training (created, changed, etc.).
    pub date: Option<DocumentDates>,
}

/// Represents a source associated with a training.
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    /// URL linking to the source.
    pub href: Option<String>,
    /// Unique identifier of the source.
    pub id: Option<i64>,
    /// Name of the source.
    pub name: Option<String>,
    /// Short name or abbreviation of the source.
    pub shortname: Option<String>,
    /// Long name of the source.
    pub longname: Option<String>,
    /// Spanish name of the source.
    pub spanish_name: Option<String>,
    /// Homepage URL of the source.
    pub homepage: Option<String>,
    /// Type descriptor of the source.
    #[serde(rename = "type")]
    pub source_type: Option<Descriptor>,
}
