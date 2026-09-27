---
paths:
  - "api/src/lib.rs"
  - "api/src/presentation/**/*.rs"
---

# Axum routing (this project pins axum 0.8.9)

- Never register the same path via two separate `.route(path, ...)` calls — axum panics at router-build time with a duplicate-route/method-overlap error. Combine methods for one path in a single call by chaining on the `MethodRouter`, e.g. `.route("/jobs", get(list_jobs_handler).post(create_job_handler))`.
- Path params use the `{param}` syntax (axum 0.8), e.g. `/jobs/{id}`, not the old `:param` syntax from earlier axum versions.
- Handlers should stay thin: extract `State<Arc<AppState>>` (+ `Path`/`Json` as needed), call exactly one use case's `execute(...)`, and return `Result<Json<T>, ApiError>` when the use case is fallible or a bare `Json<T>` when it can't fail (e.g. `list_jobs_handler`). Don't inline business logic or repository access in a handler.
