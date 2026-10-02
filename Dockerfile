# Alpine's default Rust target is musl on both amd64 and arm64, so the same
# Dockerfile produces a static binary on either architecture (build it natively).
FROM rust:1.99-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

# Build the dependencies first, so this layer is cached until Cargo.toml/Cargo.lock change.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --release --locked \
    && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release --locked


FROM scratch

COPY --from=builder /app/target/release/www-redirector /www-redirector

EXPOSE 8080

# Numeric user, so Kubernetes can verify runAsNonRoot without a passwd file.
USER 10001:10001

ENTRYPOINT ["/www-redirector"]
