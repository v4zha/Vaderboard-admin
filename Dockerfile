FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin app

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/app /usr/local/bin
COPY .env /app/.env
COPY dist /app/dist
ENTRYPOINT ["/usr/local/bin/app"]
