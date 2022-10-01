# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /lightningcss/

# needed to fix bug with nightly version not parsing cargo.toml
WORKDIR /lightningcss/
RUN sed '/serde = \[\"smallvec\/serde\", \"cssparser\/serde\"\]/d' Cargo.toml > Cargo.toml2
RUN mv Cargo.toml2 Cargo.toml

WORKDIR /lightningcss/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>
COPY --from=builder /lightningcss/fuzz/target/x86_64-unknown-linux-gnu/release/filename /
COPY --from=builder /lightningcss/fuzz/target/x86_64-unknown-linux-gnu/release/parser /
