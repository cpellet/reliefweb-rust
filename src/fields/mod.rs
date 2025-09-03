//! # Fields Module
//!
//! This module contains the typed structures for each resource returned by the ReliefWeb API.
//! Each submodule corresponds to a different resource type (e.g., `reports`, `countries`, `jobs`).
//!
//! These types are used with [`ApiResponse`] and [`ApiItem`] to deserialize API responses.
//!
//! ## Submodules
//!
//! - `common` – Shared field types used across multiple resources.
//! - `blog` – Fields for blog posts.
//! - `book` – Fields for books.
//! - `country` – Fields for countries.
//! - `disaster` – Fields for disaster events.
//! - `job` – Fields for job postings.
//! - `report` – Fields for reports.
//! - `source` – Fields for sources of information.
//! - `training` – Fields for training resources.

pub mod common;

pub mod blog;
pub mod book;
pub mod country;
pub mod disaster;
pub mod job;
pub mod report;
pub mod source;
pub mod training;
