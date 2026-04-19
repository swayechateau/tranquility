########################
# BUILDER
########################
FROM rust:latest AS builder
WORKDIR /app
ARG BIN=tquil

# Prime dependency cache
COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release || true

# Build real binary and copy it out of the cache mount
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    sh -c 'cargo build --release --locked --bin ${BIN} && cp target/release/${BIN} /app/${BIN}'

# strip for smaller size
RUN apt-get update && apt-get install -y --no-install-recommends binutils \
  && rm -rf /var/lib/apt/lists/*
RUN strip /app/${BIN} || true

########################
# RUNTIME
########################
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates tzdata libcap2 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --system --gid 10001 tquil && \
    useradd  --system --no-create-home --uid 10001 --gid 10001 --shell /usr/sbin/nologin tquil

ARG BIN=tquil
COPY --from=builder /app/${BIN} /usr/bin/${BIN}

WORKDIR /app
USER tquil:tquil
ENTRYPOINT ["/usr/bin/tquil"]
