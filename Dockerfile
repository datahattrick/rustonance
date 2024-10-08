# Build image
# Necessary dependencies to build Parrot
FROM rust:slim-bullseye as build

RUN apt-get update && apt-get install -y \
    build-essential autoconf automake cmake libtool libssl-dev pkg-config

WORKDIR "/rustonance"

# Cache cargo build dependencies by creating a dummy source
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
COPY Cargo.toml ./
COPY Cargo.lock ./
RUN cargo build --release --locked

COPY . .
RUN cargo build --release --locked

# Release image
# Necessary dependencies to run Rustonance
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y python3-pip \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
RUN pip install -U yt-dlp

COPY --from=build /rustonance/target/release/rustonance .

CMD ["./rustonance"]