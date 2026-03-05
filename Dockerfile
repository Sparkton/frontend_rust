# Stage 1: Build the frontend
FROM rust:latest AS builder
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# Copy project files
COPY . .

# Build the project
RUN cargo build --release

# Stage 2: Final image
FROM debian:bookworm-slim
WORKDIR /app

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder
COPY --from=builder /app/target/release/backend /app/backend

# Expose the port
EXPOSE 8080

# Command to run the application
CMD ["/app/backend"]
