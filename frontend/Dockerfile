ARG RUST_VERSION=1.79.0
ARG APP_NAME=EyeAP
ARG UID=10001

FROM rust:${RUST_VERSION}-alpine
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --locked trunk
COPY . /app

EXPOSE 8080

# What the container should run when it is started.
CMD ["trunk", "serve"]
