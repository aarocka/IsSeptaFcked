# IsSeptaFcked - Rust Implementation

This is the Rust implementation of the IsSeptaFcked application, a SEPTA status tracker.

## Prerequisites

- Rust 1.70 or later (install from [rustup.rs](https://rustup.rs))

## Quick Start

```bash
# Build and run in release mode
cargo build --release
cargo run --release

# Or just run in debug mode (faster compile, slower runtime)
cargo run
```

The application will start on port 5000 by default. You can override this with the `PORT` environment variable:

```bash
PORT=8080 cargo run --release
```

## Development

### Run with auto-reload on file changes:

```bash
cargo install cargo-watch
cargo watch -x run
```

### Run tests:

```bash
cargo test
```

### Check for errors without building:

```bash
cargo check
```

### Format code:

```bash
cargo fmt
```

### Lint with Clippy:

```bash
cargo clippy
```

## Project Structure

```
rust/
├── Cargo.toml           # Dependencies and project configuration
├── src/
│   ├── main.rs         # Application entry point and server setup
│   ├── routes/         # HTTP route handlers
│   │   └── mod.rs
│   ├── septa/          # SEPTA API clients and logic
│   │   ├── mod.rs      # Data structures
│   │   ├── rr.rs       # Regional Rail API client
│   │   └── bus.rs      # Bus API client
│   └── sfw.rs          # Safe-for-work filtering
├── templates/          # Tera HTML templates
│   ├── layout.html
│   ├── index.html
│   └── faq.html
└── public/            # Static assets
    ├── css/
    │   └── style.css
    ├── favicon.ico
    └── robots.txt
```

## Key Technologies

- **[Axum](https://github.com/tokio-rs/axum)** - Modern, ergonomic web framework
- **[Tokio](https://tokio.rs/)** - Async runtime for Rust
- **[Reqwest](https://github.com/seanmonstar/reqwest)** - HTTP client
- **[Tera](https://tera.netlify.app/)** - Jinja2-inspired template engine
- **[Serde](https://serde.rs/)** - Serialization/deserialization

## How It Works

1. On startup, the app spawns background tasks to fetch data from SEPTA APIs:
   - Regional Rail data is fetched every 60 seconds
   - Bus data is fetched every 5 minutes (300 seconds)

2. The fetched data is stored in thread-safe global state using `Arc<RwLock<T>>`

3. Web requests read from this cached state, ensuring fast response times

4. The age of the data is monitored, and stale data warnings are logged

## API Endpoints

- `GET /` - Main status page
- `GET /faq` - FAQ page
- `GET /echo` - Health check endpoint
- `GET /api` - API root with endpoint listing
- `GET /api/status` - Combined status summary
- `GET /api/rr` - Regional Rail full data
- `GET /api/rr/status` - Regional Rail status only
- `GET /api/rr/raw_data` - Raw SEPTA API response
- `GET /api/bus` - Bus full data
- `GET /api/bus/status` - Bus status only
- `GET /api/bus/raw_data` - Raw bus API response

## Environment Variables

- `PORT` - Port to listen on (default: 5000)
- `RUST_LOG` - Logging level (e.g., `debug`, `info`, `warn`, `error`)

Example:
```bash
RUST_LOG=debug PORT=8080 cargo run
```

## Building for Production

```bash
# Build optimized binary
cargo build --release

# The binary will be at: target/release/is_septa_fcked
./target/release/is_septa_fcked
```

## Docker Support

You can create a Dockerfile for deployment:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/is_septa_fcked /usr/local/bin/
COPY --from=builder /app/templates /templates
COPY --from=builder /app/public /public
ENV PORT=5000
CMD ["is_septa_fcked"]
```

Build and run:
```bash
docker build -t is-septa-fcked .
docker run -p 5000:5000 is-septa-fcked
```
