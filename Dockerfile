FROM rust:1.58-buster as builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM debian:buster-slim
#RUN apt-get update \
#    && apt-get install -y ca-certificates tzdata \
#    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/pushit /srv/pushit/pushit
CMD ["/srv/pushit/pushit"]