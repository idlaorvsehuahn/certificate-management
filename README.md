# Certificate Management System

A Rust-based microservices application for certificate lifecycle management, designed using production-oriented engineering practices. The system separates the write-heavy core (certificate generation and import) from the read-heavy presentation layer (inventory search and dashboards) using an event-driven architecture with separate write and read services. It uses Axum for HTTP serving, SQLx for database access on PostgreSQL, and NATS as the event broker.

---

## Project Overview

Managing X.509 TLS/SSL certificates is critical for maintaining secure machine-to-machine communications and service endpoints. Untracked or expired certificates can cause severe security vulnerabilities or unexpected service outages. 

This project provides a clean structure for issuing, importing, validating, and searching TLS certificates. By splitting responsibilities into dedicated services, it ensures that high-volume read queries (such as system monitoring or security auditing) do not impact the certificate issuance and private-key signing paths.

---

## Features

- **Certificate Generation**: A helper CA engine generates and signs development TLS certificates.
- **X.509 Parsing**: Extracts metadata (serial numbers, validation ranges, signature algorithms, public key sizes, and SAN DNS/IP entries) directly from PEM or DER inputs.
- **Asynchronous Projection**: Publishes events via NATS to synchronize database updates to the read-model projection.
- **Database Isolation**: The write-model (`certificate_db`) and read-model (`inventory_db`) are stored in isolated databases.
- **Structured Logging**: Emits structured logging parameters via the `tracing` framework.
- **Prometheus Telemetry**: Exposes performance metrics such as HTTP request counts and path latency histograms.
- **Health Verification**: Exposes a standard route to verify service liveness.

---

## Architecture Overview

The system utilizes an asynchronous event-driven design to maintain two independent databases:

```mermaid
graph TD
    Client[Client / API Gateway] -->|HTTPS| CertRouter[Certificate Service]
    Client -->|HTTPS| InvRouter[Inventory Service]
    
    subgraph Certificate Service (Write Path)
        CertRouter --> CertHandlers[HTTP Handlers]
        CertHandlers --> CertSvc[Certificate Service Logic]
        CertSvc --> CertRepo[Postgres Certificate Repository]
        CertRepo -->|Read/Write| CertDB[(PostgreSQL: certificate_db)]
        CertSvc -->|Publish Event| NatsPub[NATS Publisher]
    end
    
    NatsPub -->|Subject: certificate.issued| NatsBroker((NATS Message Broker))
    
    subgraph Inventory Service (Read Path / Projection)
        NatsBroker -->|Subscribe| NatsSub[NATS Subscriber]
        NatsSub -->|Async Project| InvSvc[Inventory Service Logic]
        InvSvc --> InvRepo[Postgres Inventory Repository]
        InvRepo -->|Write Projections| InvDB[(PostgreSQL: inventory_db)]
        
        InvRouter --> InvHandlers[HTTP Handlers]
        InvHandlers --> InvSvc
        InvRepo -->|Read Projections| InvDB
    end
    
    subgraph Shared Library (Common Code)
        shared_tls[TLS/HTTPS Layer]
        shared_metrics[Prometheus Middleware]
        shared_telemetry[Tracing Setup]
    end
    
    CertRouter & InvRouter -.->|Leverages| shared_tls
    CertRouter & InvRouter -.->|Leverages| shared_metrics
    CertRouter & InvRouter -.->|Leverages| shared_telemetry
```

---

## API Overview

### Certificate Service (Port `8080`)

| HTTP Method | Path | Description |
| :--- | :--- | :--- |
| `POST` | `/certificates` | Generates a new self-signed certificate and private key, saves it, and broadcasts an update event. |
| `POST` | `/certificates/import` | Parses and saves an existing external PEM certificate. |
| `POST` | `/certificates/parse` | Parses and returns certificate details without persisting them to the database. |
| `GET` | `/certificates/{id}` | Retrieves metadata and PEM strings for a specific certificate by UUID. |
| `GET` | `/certificates` | Lists all generated and imported certificates with pagination support. |
| `GET` | `/health` | Verifies that the service process is up. |
| `GET` | `/metrics` | Exposes Prometheus telemetry. |

### Inventory Service (Port `8081`)

| HTTP Method | Path | Description |
| :--- | :--- | :--- |
| `GET` | `/inventory` | Lists certificate inventory items with support for search, status, and expiration filtering. |
| `GET` | `/inventory/{id}` | Retrieves a single projected inventory record by UUID. |
| `GET` | `/dashboard` | Returns aggregated metrics showing the counts of active, expired, and expiring-soon certificates. |
| `GET` | `/health` | Verifies that the service process is up. |
| `GET` | `/metrics` | Exposes Prometheus telemetry. |

---

## Request Flow

When a client generates a certificate:
1. The **HTTP Request** is processed by the Axum HTTP stack, running logging, request-ID generation, and metrics middleware.
2. The **Handler** validates input formats and forwards the data transfer objects to the underlying business logic.
3. The **Service Layer** calls the CA engine to generate the certificate and private key.
4. The database repository writes the raw certificate and metadata into `certificate_db`.
5. The service dispatches an event containing metadata to the **NATS Message Broker**.
6. The client receives a successful `201 Created` response.
7. Asynchronously, the **Inventory Service NATS subscriber** receives the event, maps it, and writes the read-model projection to `inventory_db`.

---

## Project Structure

```text
├── backend
│   ├── Cargo.toml                  # Workspace definition containing shared dependencies
│   ├── docker-compose.yml          # Local infrastructure definition (PostgreSQL and NATS)
│   ├── docker                      # Deployment configurations
│   │   └── postgres                # Database startup initialization scripts
│   ├── tools                       
│   │   └── certgen                 # Dev utility to generate local SSL certificates
│   ├── crates                      # Shared internal helper crates
│   │   ├── ca-engine               # Pure Rust X.509 certificate creation and parsing
│   │   └── shared                  # Reusable HTTP metrics, telemetry, and TLS configurations
│   └── services                    
│       ├── certificate-service     # Core command service handling write operations
│       └── inventory-service       # Projection query service handling search and statistics
```

- **`crates/ca-engine`**: Keeps all low-level cryptography operations isolated. If the parsing engine needs updates, or you switch from `rcgen` to another certificate builder, only this crate needs modification.
- **`crates/shared`**: Houses the common web server infrastructure (TLS helpers, Prometheus metrics tracking, and logging). This ensures both services run with identical configuration parsers, security baselines, and instrumentation.
- **`services/`**: Independent service packages. Separating code here prevents dependency bleed and ensures developers know exactly where to make command-side vs. query-side modifications.

---

## Security

- **Local TLS (HTTPS)**: Both services are configured to communicate exclusively over TLS in development. Private keys and certificates are generated locally using the `certgen` tool and are kept ignored by Git.
- **Memory Safety**: No external C-based libraries (such as OpenSSL) are compiled or loaded. The system uses pure-Rust alternatives (`rustls` for HTTP server TLS, `ring`/`rcgen` for crypto signatures, and `x509-parser` for structure parsing).
- **Input Sanitization**: Request bodies are validated using Serde deserializer rules and custom boundary validation logic before any persistence operations occur.
- **Log Sanitation**: Sensitive information, including private key PEM strings, is explicitly filtered out of application log payloads.
- **Authentication/Authorization**: Authentication (AuthN) and authorization (AuthZ) are **intentionally not implemented** in this version and are tracked under future improvements.

---

## Engineering Trade-offs

- **Axum vs. Actix-web**: Axum was chosen because of its direct integration with the Tokio and Tower ecosystems. It allows us to leverage Tower's standardized middleware components (like `TraceLayer` and `CorsLayer`) without writing custom framework adaptors.
- **SQLx vs. ORM**: An ORM (like Diesel or SeaORM) abstracts the database behind complex type builders. SQLx was selected instead to write raw SQL queries, which are validated against our database schema at compile time. This ensures absolute query performance and safety without abstraction layers.
- **Database Segregation**: Splitting PostgreSQL databases into `certificate_db` and `inventory_db` introduces the overhead of managing multiple schemas. However, it ensures that write actions on the certificate table do not lock the read tables during high-frequency inventory searches.
- **NATS Broker**: Incorporating NATS introduces a network dependency. We chose this over simple multi-database transactions because it decouples the services—if the Inventory Service or its database experiences downtime, the Certificate Service can still issue certificates without interruption.
- **HTTPS in Development**: Enforcing HTTPS locally requires running a certificate generation step. However, it guarantees that local development is identical to production environment security assumptions, avoiding issues with mixed-content HTTP/HTTPS bindings in staging.

---

## Telemetry & Metrics

The `/metrics` endpoint on both services exposes runtime telemetry in the Prometheus format. This tracks HTTP latency histograms and status code counters. 

**Note**: Grafana and Prometheus server configurations are intentionally not bundled in this repository. The application-side endpoints are fully functional and ready to be connected to your infrastructure's existing scraping agents.

---

## Health Checks

Both services expose a `/health` endpoint. This returns a basic payload containing the service name, version, and an `"ok"` status. This is designed for use by container orchestrators (such as ECS or Kubernetes) to verify that the service process has successfully started.

---

## Running the Project

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Docker](https://www.docker.com/products/docker-desktop) and Docker Compose

### 1. Initialize Infrastructure
Spin up local PostgreSQL and NATS instances:
```bash
docker compose up -d
```
*This automatically boots up the containers and runs database setup scripts to create `certificate_db` and `inventory_db`.*

### 2. Generate Local Certificates
Generate TLS certificates for the services:
```bash
cargo run --bin certgen
```
*This outputs signed development certificates into the `backend/certs` folder.*

### 3. Launch Services
Start each service in a separate terminal:

```bash
# Terminal 1 - Certificate Service
cargo run --bin certificate-service

# Terminal 2 - Inventory Service
cargo run --bin inventory-service
```

### 4. Run Verification Tests
Ensure all unit and integration tests run successfully:
```bash
cargo test
```

---

## Future Improvements

1. **Authentication and Authorization**: Implementing JWT-based authorization to secure certificate generation and retrieval paths.
2. **Alerting System**: Adding background schedulers to send notifications (Slack or email) when certificates are nearing expiration.
3. **Background Scanning**: Running automated network sweepers to discover active SSL certificates on local network ranges and import them automatically.
4. **Monitoring Bundle**: Adding default Grafana/Prometheus configurations to the local Docker Compose file for easier local visualization.

---

## Engineering Principles

- **Explicit Errors**: Avoiding panic vectors and runtime coercion in favor of explicit, typed error propagation (`thiserror`).
- **No Unused Code**: Clean dependency tree containing only verified runtime-required crates.
- **Strict Decoupling**: Architectural boundaries separating commands, queries, events, and cryptography.