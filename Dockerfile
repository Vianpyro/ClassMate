FROM alpine:3.22
RUN apk add --no-cache ca-certificates
ARG BINARY=target/release/classmate
COPY ${BINARY} /usr/local/bin/classmate
RUN chmod +x /usr/local/bin/classmate
ENTRYPOINT ["/usr/local/bin/classmate"]
