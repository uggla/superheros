FROM rust:1.38 AS builder
RUN mkdir -p /usr/local/src
RUN groupadd -g 1000 rust \
    && useradd -u 1000 -g 1000 -s /bin/bash rust
WORKDIR /usr/local/src/superheros
COPY . .
RUN chown -R rust:rust /usr/local/src/superheros
USER rust
RUN cargo build --release
RUN cargo install --root /usr/local/src/superheros/target/release diesel_cli

FROM debian:buster-slim
ENV DEBIAN_FRONTEND noninteractive
EXPOSE 8088
RUN apt-get update && apt-get install -y \
    libmariadb3 \
    libpq5 \
    libsqlite3-0 \
    netcat \
    && rm -rf /var/lib/apt/lists/*
RUN groupadd -g 1000 superheros \
    && useradd -u 1000 -g 1000 -s /bin/bash -m superheros
WORKDIR /home/superheros
COPY --from=builder /usr/local/src/superheros/target/release/superheros .
COPY --from=builder /usr/local/src/superheros/target/release/bin/diesel .
COPY migrations migrations
COPY scripts/run.sh run.sh
COPY Cargo.toml Cargo.toml
RUN chown -R superheros:superheros /home/superheros \
    && chmod +x ./run.sh
USER superheros
CMD ["./run.sh"]
