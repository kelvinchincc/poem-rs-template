# ==============================================================================
# TEMPLATE: replace placeholders before use.
#
# Multi-stage build using cargo-chef so dependency compilation is cached:
#   chef    - Rust toolchain + cargo-chef
#   planner - computes dependency recipe (recipe.json)
#   builder - cooks dependencies (cached layer), then builds the app
#   final   - minimal distroless runtime containing only the binary
#
# Setup:
#   1. Replace every __BIN_NAME__ with your binary name (the package or [[bin]]
#      name in Cargo.toml). Example (macOS sed):
#        sed -i '' 's/__BIN_NAME__/your-binary-name/g' dockerfile
#   2. Optionally override RUST_VERSION / RUNTIME_IMAGE with --build-arg.
#   3. Add a .dockerignore that excludes target/ and .git/ to keep the build
#      context small.
#
# Build & run:
#   docker build -f dockerfile -t your-image-name .
#   docker run --rm -p 3000:3000 your-image-name   # match your app's port
#
# Notes:
#   - Distroless has no shell; keep CMD in exec form (JSON array).
#   - cc-debian12 provides glibc. Static musl binaries need a different base.
#   - The app must bind 0.0.0.0, not 127.0.0.1, to be reachable from outside
#     the container.
# ==============================================================================

ARG RUST_VERSION=1.95
ARG RUNTIME_IMAGE=gcr.io/distroless/cc-debian12

# Base: Rust toolchain + cargo-chef (installed once)
FROM rust:${RUST_VERSION}-slim-bookworm AS chef
WORKDIR /app
RUN cargo install cargo-chef

# Planner: compute the dependency recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder: compile dependencies (cached), then the app
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Final: runtime image
FROM ${RUNTIME_IMAGE}
WORKDIR /app
COPY --from=builder /app/target/release/__BIN_NAME__ .
CMD ["./__BIN_NAME__"]
