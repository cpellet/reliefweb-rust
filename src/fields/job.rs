//! # Job Endpoint Fields
//!
//! This module defines the structures for the "jobs" endpoint in the ReliefWeb API.
//! It includes the `JobFields` struct and a type alias for the `ResourceEndpoint` specialized to jobs.

use serde::{Deserialize, Serialize};

use crate::{
    endpoint::ResourceEndpoint,
    fields::common::{Descriptor, DocumentDates, Source},
};

/// Type alias for a `ResourceEndpoint` specialized for jobs.
pub type JobsEndpoint<'c> = ResourceEndpoint<'c, JobFields>;

/// Represents the fields of a job record returned by the ReliefWeb API.
#[derive(Debug, Serialize, Deserialize)]
pub struct JobFields {
    /// Unique identifier of the job.
    pub id: Option<i64>,
    /// Title of the job.
    pub title: Option<String>,
    /// Status of the job record.
    pub status: Option<String>,
    /// Job description.
    pub body: Option<String>,
    /// Instructions on how to apply for the job.
    pub how_to_apply: Option<String>,
    /// Sources associated with the job posting.
    pub source: Option<Vec<Source>>,
    /// Themes associated with the job.
    pub theme: Option<Vec<Descriptor>>,
    /// Job type.
    #[serde(rename = "type")]
    pub job_fields_type: Option<Vec<Descriptor>>,
    /// Required experience levels.
    pub experience: Option<Vec<Descriptor>>,
    /// Career categories the job belongs to.
    pub career_categories: Option<Vec<Descriptor>>,
    /// URL pointing to more information about the job.
    pub url: Option<String>,
    /// Alternative URL or alias.
    pub url_alias: Option<String>,
    /// HTML-formatted job description.
    #[serde(rename = "body-html")]
    pub body_html: Option<String>,
    /// Various dates associated with the job record (created, changed, closing, etc.).
    pub date: Option<DocumentDates>,
}
