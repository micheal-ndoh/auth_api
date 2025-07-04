# Stage 1: Build
FROM ubuntu:latest AS builder

RUN apt-get update && \
    apt-get install -y curl build-essential pkg-config libssl-dev musl-tools && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Minimal runtime image
FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/auth_api .

EXPOSE 3000
CMD ["./auth_api"] 