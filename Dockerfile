FROM jameshiew/rustbuilder:1.67.1 AS builder
RUN apt-get update && apt-get install --no-install-recommends -y \
    libpq-dev

RUN cargo install diesel_cli --no-default-features --features "postgres"


FROM debian:bookworm-slim

RUN apt-get update && apt-get install --no-install-recommends -y \
    postgresql-client

COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel