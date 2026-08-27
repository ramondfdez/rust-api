# Architecture

## Overview

The API follows a simple, layered architecture: an HTTP layer (Warp), a
handler layer that contains the business logic, and MongoDB as the
persistence layer.

```mermaid
flowchart LR
    Client["Client\n(browser / curl / Postman)"] -->|HTTP requests| Router["Warp Router\n(main.rs)"]
    Router --> Handlers["Todo Handlers\n(src/handler.rs)"]
    Handlers -->|MongoDB driver| Mongo[("MongoDB\ntodo_db")]

    subgraph "Rust API Container"
        Router
        Handlers
    end

    subgraph "MongoDB Container"
        Mongo
    end
```

## Request lifecycle

```mermaid
sequenceDiagram
    participant C as Client
    participant R as Warp Router
    participant H as Handler
    participant M as MongoDB

    C->>R: HTTP request (e.g. POST /api/todos)
    R->>H: Route match, invoke handler
    H->>M: Query / Insert / Update / Delete
    M-->>H: Result (document / error)
    H-->>R: JSON response
    R-->>C: HTTP response
```

## Components

| Component      | Responsibility                                                                    |
| -------------- | ---------------------------------------------------------------------------------- |
| `main.rs`      | Bootstraps the app: reads config, connects to MongoDB, registers routes, starts the HTTP server. |
| `handler.rs`   | Implements the CRUD handlers and the health check endpoint.                        |
| `model.rs`     | Defines the `Todo` document shape and request/query payloads.                      |
| `response.rs`  | Defines the JSON response envelopes returned to clients.                           |
| MongoDB        | Stores `Todo` documents in the `todos` collection, inside the `todo_db` database.   |

## Connecting to MongoDB

The connection is established once at startup:

1. `main.rs` reads the connection settings from environment variables
   (`MONGO_URI`, `MONGO_DB`), falling back to sane defaults for local
   development.
2. A `mongodb::Client` is created and a `ping` command is issued to verify
   connectivity before the HTTP server starts accepting traffic.
3. The resulting `Database` handle is wrapped in an `Arc` and passed into
   every handler via a Warp filter, so each request can access the `todos`
   collection.

This keeps the database connection pooled and reused across requests instead
of opening a new connection per request.

## Deployment topology (Docker Compose)

```mermaid
flowchart TB
    subgraph Host["Docker network"]
        API["api service\nport 8000:8000"]
        DB["mongodb service\nport 27017:27017"]
    end
    User["User / Client"] -->|"http://localhost:8000"| API
    API -->|"mongodb://mongodb:27017"| DB
    DB --- Volume[("mongo_data volume")]
```
</content>
