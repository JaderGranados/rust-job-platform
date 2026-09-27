# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Workspace

Cargo workspace with two members:
- `api` — the HTTP API, built with [axum](https://github.com/tokio-rs/axum). This is where all current functionality lives.
- `worker` — an unimplemented stub (`cargo new` scaffold, no dependencies). Intended for background job processing; treat any change here as greenfield.

## Commands

```sh
cargo run -p api               # start the server on http://127.0.0.1:3000
cargo test --workspace         # run all unit + integration tests
cargo test -p api              # run only the api package's tests
cargo test -p api some_test_name   # run a single test by name (substring match)
cargo test -p api --test api   # run only the integration tests in api/tests/api.rs
```

## Architecture

`api` follows a clean/hexagonal-architecture layout, with dependencies flowing inward (`presentation` → `application` → `domain`; `infrastructure` implements `domain` traits):

- `domain/` — core model and repository trait, no framework dependencies. `job.rs` holds the `Job` struct and request DTOs; `job_repository.rs` defines the `JobRepository` trait that infrastructure adapters implement.
- `application/` — one use case per file (e.g. `create_job.rs`, `get_job.rs`, `list_jobs.rs`), each a struct holding an `Arc<dyn JobRepository>` with an `execute(...)` method. This is where business rules and validation live. `error.rs` defines `ApplicationError`.
- `infrastructure/` — concrete adapters. Currently just `in_memory_repository.rs`: an `InMemoryJobRepository` backed by `Mutex<Vec<Job>>`. Data does not survive a restart; there is no database integration yet.
- `presentation/` — axum HTTP layer. `handlers.rs` holds the route handlers (extract `State<Arc<AppState>>` and call into a use case); `error.rs` maps `ApplicationError` → HTTP responses via `ApiError: IntoResponse`.
- `lib.rs` — composition root: constructs the shared repository, wires it into each use case, builds `AppState`, and registers axum routes via `build_app()`. `main.rs` just calls `build_app()` and serves it.

When adding a new use case, mirror the existing pattern: a new file in `application/`, a handler in `presentation/handlers.rs`, wiring into `AppState`/`build_app()` in `lib.rs`, and a route registration. Note axum 0.8 requires combining multiple HTTP methods on the same path into a single `.route()` call (e.g. `.route("/jobs", get(list_jobs_handler).post(create_job_handler))`) rather than registering the same path twice.

### Testing conventions

- Use-case and repository logic gets inline `#[cfg(test)] mod tests` unit tests next to the implementation (see `application/get_job.rs`, `infrastructure/in_memory_repository.rs`), typically constructing `InMemoryJobRepository::new()` directly.
- `api/tests/api.rs` has black-box integration tests that exercise real HTTP routes by building the router with `api::build_app()` and dispatching requests through `tower::ServiceExt::oneshot`. Each test gets a fresh router (no shared state between tests). Use `app.clone()` when a test needs to issue multiple sequential requests against the same in-memory state.

### Current routes

| Method | Path         | Description              |
|--------|--------------|---------------------------|
| GET    | `/`          | Hello-world placeholder   |
| GET    | `/health`    | Health check              |
| POST   | `/jobs`      | Create a job              |
| GET    | `/jobs`      | List all jobs             |
| GET    | `/jobs/{id}` | Fetch a job by id         |

`POST /jobs` expects a JSON body: `{ "job_type": "resize", "payload": 5 }`.
