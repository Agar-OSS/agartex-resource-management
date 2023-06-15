FROM rust:1.68-slim-buster as builder
WORKDIR /app/src

# Force crates.io init for better docker caching
COPY docker/caching.rs src/main.rs
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --release

COPY . .

# Force cargo to recompile
RUN touch src/main.rs
RUN cargo build --release

FROM debian:10.13-slim

WORKDIR /app
VOLUME /app/blobs

COPY --from=builder /app/src/target/release/agartex-resource-management .

EXPOSE 3200
ENTRYPOINT [ "./agartex-resource-management" ]
