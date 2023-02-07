FROM rust:1.67.0-alpine as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev git

ENV USER=www
ENV UID=1000

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --no-create-home \
  --shell "/sbin/nologin" \
  --uid "${UID}" \
  "${USER}"

WORKDIR /app

RUN cargo new www-redirector
WORKDIR /app/www-redirector
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --target x86_64-unknown-linux-musl --config net.git-fetch-with-cli=true
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl --config net.git-fetch-with-cli=true
RUN strip target/x86_64-unknown-linux-musl/release/www-redirector


FROM scratch
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
WORKDIR /app
COPY --from=builder /app/www-redirector/target/x86_64-unknown-linux-musl/release/www-redirector /app/www-redirector
USER www:www
CMD ["/app/www-redirector"]