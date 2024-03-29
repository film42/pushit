FROM rust:1.58-buster as builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update \
    && apt-get install -y libssl1.1 ca-certificates\
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/pushit /srv/pushit/pushit
COPY --from=builder /app/static /srv/pushit/static
WORKDIR /srv/pushit
CMD ["/srv/pushit/pushit"]
