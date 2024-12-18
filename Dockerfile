FROM rust:1.67 AS builder
WORKDIR /app
#WORKDIR .
COPY . .
RUN cargo install --path .

FROM ubuntu:latest
LABEL authors="shishir9159"

RUN set -x && apt-get update && apt-get install -y \
    ca-certificates curl && \
    rm -rf /var/lib/apt/lists/* \

COPY --from=builder ./adjacency.o ./.
CMD ["/app/adjacency"]