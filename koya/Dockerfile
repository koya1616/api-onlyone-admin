FROM rust:1.76.0-slim-buster as builder

WORKDIR /api
COPY . .

RUN cargo install cargo-make
RUN cargo install --path .

CMD ["cargo", "make", "watch"]