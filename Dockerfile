FROM rust:1.76.0-slim-buster as builder

WORKDIR /api
COPY . .

RUN cargo build --release

FROM debian:buster-slim as runner

COPY --from=builder /api/target/release/api_onlyone_admin /usr/local/bin/api_onlyone_admin
WORKDIR /usr/local/bin

CMD ["api_onlyone_admin"]