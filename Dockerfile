# Use a Rust base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build the dependencies (but not the actual app)
RUN mkdir src && \
    echo "fn main() { println!(\"placeholder\") }" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the source code into the container
COPY . .

# Build the application with maximum optimizations
RUN cargo build --release

# Start a new stage for the final image
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/cache_fr_server .

# Expose any necessary ports
# EXPOSE 8080

# Command to run the executable
CMD ["./cache_fr_server"]
