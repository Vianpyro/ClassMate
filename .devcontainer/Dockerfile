FROM rust:alpine3.22

RUN apk add --no-cache \
    build-base \
    musl-dev \
    linux-headers \
    pkgconf

CMD ["/bin/sh", "-lc", "rustc --version && cargo --version"]
