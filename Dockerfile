FROM rust:1.76 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/mcp03 .
EXPOSE 5555
CMD ["./mcp03"]