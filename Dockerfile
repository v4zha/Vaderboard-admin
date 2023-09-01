FROM rust:latest as builder

WORKDIR /app

RUN apt-get update && apt-get install -y libsqlite3-dev && cargo install sqlx-cli --no-default-features --features sqlite

COPY . .


RUN sqlx database create && \
    sqlx migrate run
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y sqlite3

WORKDIR /app

COPY --from=builder /app/target/release/your_project_name /app/your_project_name

EXPOSE 8080

CMD ["/app/"]
