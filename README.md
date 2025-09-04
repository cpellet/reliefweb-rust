# ReliefWeb Rust Client

[![crates.io](https://img.shields.io/crates/v/reliefweb)](https://crates.io/crates/reliefweb) 
[![docs.rs](https://docs.rs/reliefweb/badge.svg)](https://docs.rs/reliefweb)

A fully asynchronous Rust client for the [ReliefWeb API](https://api.reliefweb.int/), providing typed endpoints for reports, disasters, countries, jobs, blogs, books, sources, and trainings.

---

## Features

- Fully typed resources with serde support.
- Async support with `reqwest` + `tokio`.
- Support for filtering, sorting, limiting, and profiles.
- Prebuilt endpoints for:
  - `reports`
  - `disasters`
  - `countries`
  - `jobs`
  - `training`
  - `sources`
  - `blogs`
  - `books`
- Automatic URL and query parameter handling.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
reliefweb-rust = "0.1.0"
tokio = { version = "1.47.1", features = ["full"] }
```

## Usage

```rust
use reliefweb_rust::{Client, QueryParams, APIVersion};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("api.reliefweb.int", "my_app_name", APIVersion::V2)?;

    // List reports
    let reports = client.reports()
        .list(Some(&QueryParams::new().limit(5)))
        .await?;

    for report in reports.data {
        println!("Report: {:?}", report.fields.title);
    }

    // Get a single report
    if let Some(first_report) = reports.data.first() {
        let report_detail = client.reports()
            .get(&first_report.id, None, None, None)
            .await?;
        println!("Full report details: {:?}", report_detail.data[0].fields);
    }

    Ok(())
}
```

## QueryParams
You can filter, sort, and limit results using `QueryParams`:
```rust
use reliefweb_rust::{QueryParams, QueryProfile};

let params = QueryParams::new()
    .limit(10)
    .profile(QueryProfile::Minimal);

```

## Documentation
Full API documentation is available at [docs.rs](https://docs.rs/reliefweb)

## License
Licensed under MIT.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.