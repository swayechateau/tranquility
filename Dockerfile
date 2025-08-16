# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev

# Copy source code
COPY . .

# Build the Rust CLI app in release mode
RUN cargo build --release

# Runtime stage
FROM alpine:latest

WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/tranquility /app/tquil

# Set entrypoint to the CLI app
ENTRYPOINT ["/app/tquil"]