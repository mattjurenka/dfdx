# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /dfdx/

WORKDIR /dfdx/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>

COPY --from=builder /dfdx/fuzz/target/x86_64-unknown-linux-gnu/release/relu /

COPY --from=builder /dfdx/fuzz/target/x86_64-unknown-linux-gnu/release/math /

COPY --from=builder /dfdx/fuzz/target/x86_64-unknown-linux-gnu/release/stats /

COPY --from=builder /dfdx/fuzz/target/x86_64-unknown-linux-gnu/release/relu_large /

COPY --from=builder /dfdx/fuzz/target/x86_64-unknown-linux-gnu/release/math_large /

COPY --from=builder /dfdx/fuzz/target/x86_64-unknown-linux-gnu/release/stats_large /