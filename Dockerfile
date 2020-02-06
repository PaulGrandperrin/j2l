FROM rust as builder

RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /j2l
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM gcr.io/distroless/cc
COPY --from=builder /j2l/target/x86_64-unknown-linux-musl/release/j2l /bin/j2l
ENTRYPOINT ["/bin/j2l"]
