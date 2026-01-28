FROM rust:1.75 as builder

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Create the runtime image
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/is_septa_fcked /usr/local/bin/

# Copy templates and public files
COPY templates /templates
COPY public /public

# Set the port environment variable
ENV PORT=5000
ENV RUST_LOG=info

# Expose the port
EXPOSE 5000

# Run the application
CMD ["is_septa_fcked"]
