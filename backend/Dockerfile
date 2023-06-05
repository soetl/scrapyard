FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/backend
COPY . .

RUN apt-get update && apt-get install libpq-dev -y
RUN cargo install --path . --no-default-features

FROM debian:bullseye-slim as runtime

RUN apt-get update && apt-get install -y libpq-dev && apt install -y openssl

COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend