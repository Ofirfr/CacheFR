# Use a more stable Debian base image for both build and runtime stages
FROM debian:buster-slim as base

# Install required dependencies for building the application
RUN apt-get update && \
    apt-get install -y \
    curl \
    cmake \
    build-essential \
    libssl-dev \
    pkg-config && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Rust toolchain using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build the dependencies (but not the actual app)
RUN mkdir src && \
    echo "fn main() { println!(\"placeholder\") }" > src/server.rs && \
    cargo build --release && \
    rm -rf src

# Copy the source code into the container
COPY . .

# Build the application with maximum optimizations
RUN cargo build --release

# Use the same base image for the final stage to ensure glibc compatibility
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=base /usr/src/app/target/release/cache_fr_server .

# Expose the necessary port
EXPOSE 50051

# Command to run the executable
CMD ["./cache_fr_server"]
