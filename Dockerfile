# Build stage
FROM golang:1.25-alpine AS builder

WORKDIR /app

# Copy go mod files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy source code
COPY . .

# Build the binary
RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-w -s" -o /asc-mcp ./cmd/asc-mcp

# Runtime stage
FROM alpine:3.19

# Add CA certificates for HTTPS
RUN apk --no-cache add ca-certificates

WORKDIR /

# Copy binary from builder
COPY --from=builder /asc-mcp /asc-mcp

# Run as non-root user
RUN adduser -D -g '' appuser
USER appuser

ENTRYPOINT ["/asc-mcp"]
