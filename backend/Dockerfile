FROM rust:1.86 as builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libpq-dev && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

COPY Cargo.toml ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/bookmark_manager*

RUN rm -rf src

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

COPY --from=builder /app/target/release/bookmark_manager ./

COPY migrations ./migrations

RUN echo "DATABASE_URL=postgresql://postgres:password@postgres:5432/bookmark_manager" > .env

EXPOSE 8000

CMD ["./bookmark_manager"]