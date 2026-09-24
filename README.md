# rust-job-platform

A small job platform API written in Rust, structured as a Cargo workspace with a clean-architecture-style layout (domain / application / infrastructure / presentation).

## Workspace

- `api` — the HTTP API, built with [axum](https://github.com/tokio-rs/axum).
- `worker` — currently an unimplemented stub (`cargo new` scaffold, no dependencies). Intended for background job processing.

## Running

```sh
cargo run -p api
```

The server listens on `http://127.0.0.1:3000`.

## Routes

| Method | Path         | Description                     |
|--------|--------------|----------------------------------|
| GET    | `/`          | Hello-world placeholder          |
| GET    | `/health`    | Health check                     |
| POST   | `/jobs`      | Create a job                     |
| GET    | `/jobs/{id}` | Fetch a job by id                |

`POST /jobs` expects a JSON body:

```json
{ "job_type": "resize", "payload": 5 }
```

## Persistence

Jobs are stored **in memory** (`Mutex<Vec<Job>>`) — data does not survive a restart. There is no database integration yet.

## Testing

```sh
cargo test --workspace
```

Includes unit tests for the domain/application logic and integration tests (`api/tests/api.rs`) that exercise the real HTTP routes via `tower::ServiceExt::oneshot`.
