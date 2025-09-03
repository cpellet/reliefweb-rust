#[cfg(test)]
mod integration_tests {
    use reliefweb::{
        Client, RELIEFWEB_DOMAIN, ResourceEndpoint, blog::BlogsEndpoint, book::BooksEndpoint,
        country::CountriesEndpoint, disaster::DisastersEndpoint, job::JobsEndpoint,
        report::ReportsEndpoint, source::SourcesEndpoint, training::TrainingsEndpoint,
    };

    fn client() -> Client {
        let app_name = "reliefweb_rust_client_tests";
        Client::new(RELIEFWEB_DOMAIN, app_name, reliefweb::APIVersion::V2).unwrap()
    }

    async fn test_list_and_get<T>(endpoint: &ResourceEndpoint<'_, T>)
    where
        T: serde::de::DeserializeOwned,
    {
        let params = reliefweb::QueryParams::new()
            .limit(5)
            .profile(reliefweb::QueryProfile::Minimal);

        let list_resp = endpoint.list(Some(&params)).await.unwrap();
        assert!(!list_resp.data.is_empty(), "list returned empty data");

        let first_id = &list_resp.data[0].id;
        let get_resp = endpoint
            .get(first_id, Some(reliefweb::QueryProfile::Minimal), None, None)
            .await
            .unwrap();
        assert_eq!(get_resp.data[0].id, *first_id, "get returned wrong id");
    }

    #[tokio::test]
    #[ignore]
    async fn reports() {
        let client = client();
        let endpoint = ReportsEndpoint::new(&client, "reports");
        test_list_and_get(&endpoint).await;
    }

    #[tokio::test]
    #[ignore]
    async fn countries() {
        let client = client();
        let endpoint = CountriesEndpoint::new(&client, "countries");
        test_list_and_get(&endpoint).await;
    }

    #[tokio::test]
    #[ignore]
    async fn disasters() {
        let client = client();
        let endpoint = DisastersEndpoint::new(&client, "disasters");
        test_list_and_get(&endpoint).await;
    }

    #[tokio::test]
    #[ignore]
    async fn jobs() {
        let client = client();
        let endpoint = JobsEndpoint::new(&client, "jobs");
        test_list_and_get(&endpoint).await;
    }

    #[tokio::test]
    #[ignore]
    async fn training() {
        let client = client();
        let endpoint = TrainingsEndpoint::new(&client, "training");
        test_list_and_get(&endpoint).await;
    }

    #[tokio::test]
    #[ignore]
    async fn sources() {
        let client = client();
        let endpoint = SourcesEndpoint::new(&client, "sources");
        test_list_and_get(&endpoint).await;
    }

    #[tokio::test]
    #[ignore]
    async fn blog() {
        let client = client();
        let endpoint = BlogsEndpoint::new(&client, "blog");
        test_list_and_get(&endpoint).await;
    }

    #[tokio::test]
    #[ignore]
    async fn book() {
        let client = client();
        let endpoint = BooksEndpoint::new(&client, "book");
        test_list_and_get(&endpoint).await;
    }
}
