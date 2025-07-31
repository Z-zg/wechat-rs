# 构建阶段
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates

RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 复制编译好的二进制文件
COPY --from=builder /app/target/release/my-rust-project ./app
COPY --from=builder /app/templates ./templates

EXPOSE 3000

CMD ["./app"]