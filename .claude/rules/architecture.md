# Layering rules (api package)

Dependencies flow one way: `presentation` → `application` → `domain`. `infrastructure` implements `domain` traits and is depended on by the composition root only.

- `domain/` must stay framework-free — no axum, no tokio, no serde-specific HTTP concerns. Only the core model (`job.rs`) and repository trait (`job_repository.rs`).
- `application/` holds one use case per file, each a struct wrapping `Arc<dyn JobRepository>` with an `execute(...)` method. Business rules and validation live here, not in handlers. Errors surface as `ApplicationError` (`application/error.rs`), never raw axum types.
- `infrastructure/` holds concrete adapters (currently `InMemoryJobRepository`, backed by `Mutex<Vec<Job>>`). New persistence backends go here behind the `JobRepository` trait — don't leak storage details into `application` or `presentation`.
- `presentation/` is the only layer allowed to know about axum. Handlers extract `State<Arc<AppState>>`, call a single use case, and map `ApplicationError` → `ApiError` (`presentation/error.rs`) via `?`/`From`.
- `lib.rs` is the composition root: it builds the shared repository `Arc`, wires every use case from clones of it, assembles `AppState`, and registers routes in `build_app()`. `main.rs` only calls `build_app()` and serves it — don't put business logic there.

When adding a new use case: new file in `application/` mirroring `get_job.rs`'s shape, a handler in `presentation/handlers.rs`, a field on `AppState`, and wiring in `build_app()` (clone the repository `Arc` for every use case except the last one, which can move it).

`worker` is an unimplemented stub (no dependencies) — treat any work there as greenfield, not an extension of existing patterns.
