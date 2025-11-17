FROM rust:alpine3.22

RUN apk add --no-cache \
    build-base \
    musl-dev \
    linux-headers \
    pkgconf

RUN addgroup -S vscode && adduser -S -G vscode vscode
USER vscode

# Use rustc --version as a simple presence check for the dev image.
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD /bin/sh -c "rustc --version >/dev/null 2>&1 || exit 1"

CMD ["/bin/sh", "-lc", "rustc --version && cargo --version"]
