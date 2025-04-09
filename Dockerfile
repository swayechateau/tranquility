# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY cli/Cargo.toml cli/Cargo.lock ./

# This build step will cache dependencies
RUN cargo build --release

# Copy the source code
COPY cli/. .

# Build the application
RUN cargo install --path .

# Set the startup command to run the binary
CMD ["app"]