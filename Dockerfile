# Use an official Rust image as the base
FROM rust:1.68-slim-buster as builder

# Set the working directory
WORKDIR /usr/src/weather_mock_api

# Copy the source code and build the application
COPY . .
RUN cargo build --release

# Use a minimal Debian-based image to reduce the container size
FROM debian:buster-slim

# Install any necessary dependencies (openssl and ca-certificates)
RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder image
COPY --from=builder /usr/src/weather_mock_api/target/release/weather_mock_api /usr/local/bin/weather_mock_api

# Set the working directory
WORKDIR /usr/src/app

# Expose the port the app runs on
EXPOSE 8080

# Start the application
CMD ["weather_mock_api"]