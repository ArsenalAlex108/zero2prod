pub fn get_mock_builder() -> wiremock::MockBuilder {
    wiremock::Mock::given(wiremock::matchers::path(
        "/email",
    ))
    .and(wiremock::matchers::method("POST"))
}
