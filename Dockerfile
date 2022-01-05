FROM alpine:latest AS build

RUN apk update
RUN apk add build-base curl
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
#COPY migrations/ ./migrations
RUN source $HOME/.cargo/env && cargo build --release
RUN strip ./target/release/soundy-v2

FROM scratch

COPY --from=build /build/target/release/soundy-v2 /usr/local/bin/soundy-v2
CMD ["soundy-v2"]