FROM clux/muslrust:stable as builder

RUN apt-get update &&\
    apt-get install -y ca-certificates curl gnupg &&\
    mkdir -p /etc/apt/keyrings &&\
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg &&\
    echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_18.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list 

RUN apt-get update && apt-get install -y nodejs  

WORKDIR /app

RUN apt-get install -y libsqlite3-dev && cargo install sqlx-cli --no-default-features --features sqlite

COPY . .

RUN cd vader-admin-ui && npm install

RUN sqlx database create && \
    sqlx migrate run

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y sqlite3

WORKDIR /app

VOLUME ["/app/data"]

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/vader-admin /app/
COPY --from=builder /app/.env /app/
COPY --from=builder /app/dist/ /app/dist/
COPY --from=builder /app/data/vaderboard.db /app/data/

EXPOSE 8080

CMD ["/app/vader-admin"]
