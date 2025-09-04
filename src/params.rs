use std::fmt;

use reqwest::Url;

/// `QueryProfile` specifies which sets of fields to include in result.
#[derive(Default)]
pub enum QueryProfile {
    /// Just the `title` or `name` field
    #[default]
    Minimal,
    /// All fields
    Full,
    /// Results suited to lists and tables
    List,
}

impl fmt::Display for QueryProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryProfile::Minimal => write!(f, "minimal"),
            QueryProfile::Full => write!(f, "full"),
            QueryProfile::List => write!(f, "list"),
        }
    }
}

/// A shorthand specification of sets of fields, filters and sort order for common use-cases. Similar to `profile` but with more opinions
#[derive(Default)]
pub enum QueryPreset {
    ///The default setting applies sensible status filters for most requests
    #[default]
    Minimal,
    ///Sorts by date for appropriate content types. Countries and sources sorted by id. Use this for list requests.
    Latest,
    /// Include archived disasters and expired jobs and training which otherwise would be excluded from results. This is useful for visualizing trends and performing analysis of past content.
    Analysis,
}

impl fmt::Display for QueryPreset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryPreset::Minimal => write!(f, "minimal"),
            QueryPreset::Latest => write!(f, "latest"),
            QueryPreset::Analysis => write!(f, "analysis"),
        }
    }
}

#[derive(Default)]
/// Specifies how to interpret spaces in queries. Can be AND or OR. Default value is OR.
pub enum FilterOperator {
    #[default]
    OR,
    AND,
}

impl fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FilterOperator::AND => write!(f, "AND"),
            FilterOperator::OR => write!(f, "OR"),
        }
    }
}

/// Specifies a full-text filter for the query
pub struct QueryQuery {
    /// What to search for. Required for all queries.
    pub value: String,
    /// Which fields to query on. See [field tables](https://apidoc.reliefweb.int/fields-tables) for options.
    pub fields: Vec<String>,
    /// How to interpret spaces in the query. Can be AND or OR. Default value is OR.
    pub operator: Option<FilterOperator>,
}

/// `Narrows down the content to be searched in. These correspond to the 'refine' section of the search bar.
pub struct QueryFilter {
    ///Which field to filter on. See [field tables](https://apidoc.reliefweb.int/fields-tables).
    pub field: String,
    /// The value to filter for. Most of the possible values are pre-defined. If this is for a `date`, or numeric value (e.g. `id`), it can be a range defined by `from` and `to` values. If only `from` or `to` is present, then value will match those greater than or equal to or less than or equal to the value respectively. If `value` is missing, the filter will act on whether the field exists or not.
    pub value: String,
    /// How to combine filter array values or conditions. Can be AND or OR.
    pub operator: Option<FilterOperator>,
    /// Set to `true` to select all items that do not match the filter.
    pub negate: bool,
}

/// Specifies the sorting direction of results for a given field.
#[derive(Default)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SortDirection::Asc => write!(f, "asc"),
            SortDirection::Desc => write!(f, "desc"),
        }
    }
}

/// Specifies how results should be sorted for a given field.
pub struct SortDescriptor {
    pub field: String,
    pub direction: SortDirection,
}

// Query parameters for filtering, sorting, and field selection.
///
/// Provides a builder-style API to chain filters, queries, sorting, and inclusion/exclusion of fields.
///
/// # Example
///
/// ```no_run
/// use reliefweb::{QueryParams, QueryProfile};
///
/// let params = QueryParams::new()
///     .limit(10)
///     .profile(QueryProfile::Minimal)
///     .include(vec!["title".to_string(), "source".to_string()]);
/// ```
#[derive(Default)]
pub struct QueryParams {
    /// Free-text search in given fields.
    pub query: Vec<QueryQuery>,
    /// Narrows down content to be searched in. Corresponds to the 'refine' section in the web UI.
    pub filter: Vec<QueryFilter>,
    ///A helper for creating correct API calls, setting verbose=1 adds a details section to the response to display the query parameters as a JSON object.
    ///
    /// This is for checking how the GET parameters are translated into JSON, or that the POST parameters sent are as intended.
    pub verbose: Option<bool>,
    ///How many results to return. Must be between 0 and 1000, defaults to 10.
    pub limit: Option<u32>,
    /// How many results to skip. Used for paging through results or for getting more than the limit. Must be greater than 0, defaults to 0.
    pub offset: Option<u32>,
    ///  Sortable field names and their direction `desc` or `asc` in the form `field:order`. The order of the field names in the array will determine the priority of the sorting.
    ///
    /// Note: requests with a query will be sorted by relevance. If there is no `sort` specified, results may not be consistent.
    ///
    /// For the most recent results, use the preset `latest`.
    pub sort: Vec<SortDescriptor>,
    /// A shorthand specification for which sets of fields to include in result.
    pub profile: Option<QueryProfile>,
    /// A shorthand specification of sets of fields, filters and sort order for common use-cases. Similar to profile but with more opinions.
    pub preset: Option<QueryPreset>,
    /// Arrays of fields to return in the result. To be used in conjunction with the profile parameter to personalize the fields returned and streamline requests.
    pub include: Vec<String>,
    /// Arrays of fields to exclude from the result. To be used in conjunction with the profile parameter to personalize the fields returned and streamline requests.
    pub exclude: Vec<String>,
}

impl QueryParams {
    /// Create a default set of query parameters.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(mut self, query: QueryQuery) -> Self {
        self.query.push(query);
        self
    }

    pub fn queries(mut self, queries: Vec<QueryQuery>) -> Self {
        self.query.extend(queries);
        self
    }

    pub fn filter(mut self, filter: QueryFilter) -> Self {
        self.filter.push(filter);
        self
    }

    pub fn filters(mut self, filters: Vec<QueryFilter>) -> Self {
        self.filter.extend(filters);
        self
    }

    pub fn verbose(mut self, v: bool) -> Self {
        self.verbose = Some(v);
        self
    }

    pub fn limit(mut self, l: u32) -> Self {
        self.limit = Some(l);
        self
    }

    pub fn offset(mut self, o: u32) -> Self {
        self.offset = Some(o);
        self
    }

    pub fn sort(mut self, sort: Vec<SortDescriptor>) -> Self {
        self.sort.extend(sort);
        self
    }

    pub fn profile(mut self, profile: QueryProfile) -> Self {
        self.profile = Some(profile);
        self
    }

    pub fn preset(mut self, preset: QueryPreset) -> Self {
        self.preset = Some(preset);
        self
    }

    pub fn include(mut self, include: Vec<String>) -> Self {
        self.include.extend(include);
        self
    }

    pub fn exclude(mut self, exclude: Vec<String>) -> Self {
        self.exclude.extend(exclude);
        self
    }
}

impl QueryParams {
    /// Add URL-encoded params from a given existing Url
    pub(crate) fn apply_to_url(&self, url: &mut Url) {
        let mut qp = url.query_pairs_mut();

        if let Some(v) = self.verbose {
            qp.append_pair("verbose", if v { "1" } else { "0" });
        }
        if let Some(l) = self.limit {
            qp.append_pair("limit", &l.to_string());
        }
        if let Some(o) = self.offset {
            qp.append_pair("offset", &o.to_string());
        }
        if let Some(profile) = &self.profile {
            qp.append_pair("profile", &profile.to_string());
        }
        if let Some(preset) = &self.preset {
            qp.append_pair("preset", &preset.to_string());
        }

        for inc in &self.include {
            qp.append_pair("fields[include][]", inc);
        }
        for exc in &self.exclude {
            qp.append_pair("fields[exclude][]", exc);
        }

        for (i, q) in self.query.iter().enumerate() {
            let prefix = format!("query[{i}]");
            qp.append_pair(&format!("{prefix}[value]"), &q.value);
            for (j, field) in q.fields.iter().enumerate() {
                qp.append_pair(&format!("{prefix}[fields][{j}]"), field);
            }
            if let Some(op) = &q.operator {
                qp.append_pair(&format!("{prefix}[operator]"), &op.to_string());
            }
        }

        if !self.filter.is_empty() {
            if self.filter.iter().any(|f| f.operator.is_some()) {
                let top_op = self
                    .filter
                    .iter()
                    .filter_map(|f| f.operator.as_ref())
                    .next()
                    .unwrap()
                    .to_string();
                qp.append_pair("filter[operator]", &top_op);
            }
            for (i, f) in self.filter.iter().enumerate() {
                qp.append_pair(&format!("filter[conditions][{i}][field]"), &f.field);
                qp.append_pair(&format!("filter[conditions][{i}][value][]"), &f.value);
                if f.negate {
                    qp.append_pair(&format!("filter[conditions][{i}][negate]"), "1");
                }
                if let Some(op) = &f.operator {
                    qp.append_pair(
                        &format!("filter[conditions][{i}][operator]"),
                        &op.to_string(),
                    );
                }
            }
        }

        for s in &self.sort {
            qp.append_pair("sort[]", &format!("{}:{}", s.field, s.direction));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Url;

    #[test]
    fn test_query_params_builder() {
        let qp = QueryParams::new()
            .verbose(true)
            .limit(50)
            .offset(10)
            .profile(QueryProfile::Full)
            .preset(QueryPreset::Analysis)
            .include(vec!["field1".into(), "field2".into()])
            .exclude(vec!["field3".into()])
            .query(QueryQuery {
                value: "search".into(),
                fields: vec!["title".into()],
                operator: Some(FilterOperator::AND),
            })
            .filter(QueryFilter {
                field: "status".into(),
                value: "active".into(),
                operator: Some(FilterOperator::OR),
                negate: false,
            })
            .sort(vec![SortDescriptor {
                field: "date".into(),
                direction: SortDirection::Desc,
            }]);

        assert_eq!(qp.verbose, Some(true));
        assert_eq!(qp.limit, Some(50));
        assert_eq!(qp.offset, Some(10));
        assert_eq!(qp.profile.unwrap().to_string(), "full");
        assert_eq!(qp.preset.unwrap().to_string(), "analysis");
        assert_eq!(qp.include, vec!["field1", "field2"]);
        assert_eq!(qp.exclude, vec!["field3"]);
        assert_eq!(qp.query.len(), 1);
        assert_eq!(qp.filter.len(), 1);
        assert_eq!(qp.sort.len(), 1);
    }

    #[test]
    fn test_apply_to_url_basic() {
        let mut url = Url::parse("https://example.com/api").unwrap();

        let qp = QueryParams::new().verbose(true).limit(25).offset(5);

        qp.apply_to_url(&mut url);

        let query: Vec<(_, _)> = url.query_pairs().collect();
        assert!(query.contains(&("verbose".into(), "1".into())));
        assert!(query.contains(&("limit".into(), "25".into())));
        assert!(query.contains(&("offset".into(), "5".into())));
    }

    #[test]
    fn test_apply_to_url_include_exclude() {
        let mut url = Url::parse("https://example.com/api").unwrap();

        let qp = QueryParams::new()
            .include(vec!["title".into(), "summary".into()])
            .exclude(vec!["internal".into()]);

        qp.apply_to_url(&mut url);

        let query: Vec<(_, _)> = url.query_pairs().collect();
        assert!(query.contains(&("fields[include][]".into(), "title".into())));
        assert!(query.contains(&("fields[include][]".into(), "summary".into())));
        assert!(query.contains(&("fields[exclude][]".into(), "internal".into())));
    }

    #[test]
    fn test_apply_to_url_queries() {
        let mut url = Url::parse("https://example.com/api").unwrap();

        let qp = QueryParams::new().query(QueryQuery {
            value: "foo".into(),
            fields: vec!["title".into(), "content".into()],
            operator: Some(FilterOperator::AND),
        });

        qp.apply_to_url(&mut url);

        let query: Vec<(_, _)> = url.query_pairs().collect();
        assert!(query.contains(&("query[0][value]".into(), "foo".into())));
        assert!(query.contains(&("query[0][fields][0]".into(), "title".into())));
        assert!(query.contains(&("query[0][fields][1]".into(), "content".into())));
        assert!(query.contains(&("query[0][operator]".into(), "AND".into())));
    }

    #[test]
    fn test_apply_to_url_filters() {
        let mut url = Url::parse("https://example.com/api").unwrap();

        let qp = QueryParams::new().filter(QueryFilter {
            field: "status".into(),
            value: "active".into(),
            operator: Some(FilterOperator::OR),
            negate: true,
        });

        qp.apply_to_url(&mut url);

        let query: Vec<(_, _)> = url.query_pairs().collect();
        assert!(query.contains(&("filter[operator]".into(), "OR".into())));
        assert!(query.contains(&("filter[conditions][0][field]".into(), "status".into())));
        assert!(query.contains(&("filter[conditions][0][value][]".into(), "active".into())));
        assert!(query.contains(&("filter[conditions][0][negate]".into(), "1".into())));
    }

    #[test]
    fn test_apply_to_url_sort() {
        let mut url = Url::parse("https://example.com/api").unwrap();

        let qp = QueryParams::new().sort(vec![SortDescriptor {
            field: "date".into(),
            direction: SortDirection::Desc,
        }]);

        qp.apply_to_url(&mut url);

        let query: Vec<(_, _)> = url.query_pairs().collect();
        assert!(query.contains(&("sort[]".into(), "date:desc".into())));
    }
}
