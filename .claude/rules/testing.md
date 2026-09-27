# Testing conventions

- Unit-test use cases and repository logic inline with `#[cfg(test)] mod tests` next to the implementation (see `application/get_job.rs`, `infrastructure/in_memory_repository.rs`). Construct `InMemoryJobRepository::new()` directly rather than mocking `JobRepository`.
- Integration tests live in `api/tests/api.rs` as black-box HTTP tests: build the router with `api::build_app()` and dispatch via `tower::ServiceExt::oneshot`. Use the existing `app()` and `json_body(response)` helpers.
- Each test gets its own fresh router/in-memory state — there is no shared state across tests. If a test needs multiple sequential requests against the same state, call `app.clone()` per request rather than rebuilding the router.
- Run the whole suite with `cargo test --workspace`; scope to the API package with `cargo test -p api`, or to just the integration tests with `cargo test -p api --test api`.
