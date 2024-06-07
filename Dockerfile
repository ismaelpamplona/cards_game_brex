# Use the official Rust image as a base
FROM rust:latest

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code and tests
COPY src ./src
COPY tests ./tests

# Build dependencies in a separate layer to cache them
RUN cargo build --release && rm -rf target

# Build the application
RUN cargo build --release

# Run the tests
CMD ["cargo", "test"]