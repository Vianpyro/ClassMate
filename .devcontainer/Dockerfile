FROM rust:alpine3.22

RUN apk add --no-cache \
    build-base \
    musl-dev \
    linux-headers \
    pkgconf

RUN addgroup -S vscode && adduser -S -G vscode vscode
USER vscode

CMD ["/bin/sh", "-lc", "rustc --version && cargo --version"]
