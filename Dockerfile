FROM rust:latest AS builder
WORKDIR app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/time-dependent-quotes actix-backend
COPY --from=builder /app/migrations migrations
COPY --from=builder /app/templates templates
RUN mkdir tmp
RUN mkdir data
ENTRYPOINT ["./actix-backend"]