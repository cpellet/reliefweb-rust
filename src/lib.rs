//! # ReliefWeb Rust Client
//!
//! This crate provides an unofficial strongly-typed Rust client for OCHA's [ReliefWeb API](https://apidoc.reliefweb.int/).
//!
//! It supports listing and getting various resources (`reports`, `disasters`, `countries`, `jobs`, etc.) with typed queries, filtering, field selection, and more.
//!
//!# Example
//!
//! ```no_run
//! use reliefweb::{APIVersion, Client, QueryParams, RELIEFWEB_DOMAIN};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new(RELIEFWEB_DOMAIN, "my_descriprive_app_name", APIVersion::V2).unwrap();
//!     
//!     let reports = client.reports()
//!         .list(Some(&QueryParams::new().limit(5)))
//!         .await
//!         .unwrap();
//!     
//!     println!("Got {} reports", reports.data.len());
//! }

mod client;
mod endpoint;
mod fields;
mod params;
mod response;

pub use client::*;
pub use endpoint::*;
pub use fields::*;
pub use params::*;
pub use response::*;
