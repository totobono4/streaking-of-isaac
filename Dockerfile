# --- build ---
FROM rust:1.80-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev sqlite3 libsqlite3-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

# --- runtime ---
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libsqlite3-0 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/streaking_of_isaac /app/streaking_of_isaac
COPY --from=builder /app/target/release/create_admin /app/create_admin
COPY static ./static
COPY migrations ./migrations
ENV DATABASE_URL=sqlite:///data/data.db
VOLUME ["/data"]
EXPOSE 8080
CMD ["/app/streaking_of_isaac"]
