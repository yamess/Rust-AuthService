ARG RUST_VERSION=1.76.0
ARG APP_NAME=crud

FROM rust:${RUST_VERSION}-slim-bullseye as build

USER root

ARG APP_NAME

WORKDIR /$APP_NAME

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    <<EOF
    set -e
    cargo build --locked --release
    cp ./target/release/$APP_NAME /bin/server
EOF


FROM debian:bullseye-slim as final

ARG UID=10001

RUN adduser \
    --disabled-password \
    --gecos '' \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid $UID \
    appuser

USER appuser

COPY --from=build /bin/server /bin/

CMD ["bin/server"]