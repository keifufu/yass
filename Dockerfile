FROM rust:alpine3.18 AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add pkgconfig openssl-dev musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.18
RUN apk add pkgconfig openssl-dev libgcc
WORKDIR /app
COPY --from=builder /app/target/release/yass /app/
RUN chmod +x /app/yass
ENTRYPOINT ["/app/yass"]