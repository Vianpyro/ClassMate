FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

ARG BINARY=target/release/classmate
COPY ${BINARY} /usr/local/bin/classmate
RUN chmod +x /usr/local/bin/classmate
ENTRYPOINT ["/usr/local/bin/classmate"]
