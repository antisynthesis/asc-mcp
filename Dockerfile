# Build stage: a static musl binary.
FROM rust:1.97-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

# Cache dependency builds: compile a placeholder crate against the real
# manifest first, then the actual sources.
COPY Cargo.toml Cargo.lock build.rs ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && touch src/lib.rs \
    && cargo build --release --locked && rm -rf src

COPY src ./src
RUN touch src/main.rs src/lib.rs && cargo build --release --locked \
    && cp target/release/asc-mcp /asc-mcp

# Runtime stage
FROM alpine:3.20

# CA certificates for HTTPS to the App Store Connect API
RUN apk --no-cache add ca-certificates

WORKDIR /

COPY --from=builder /asc-mcp /asc-mcp

# Run as non-root user
RUN adduser -D -g '' appuser
USER appuser

ENTRYPOINT ["/asc-mcp"]
