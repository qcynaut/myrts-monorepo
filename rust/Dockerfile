FROM rust:1.89.0-slim-bullseye AS builder

WORKDIR /app

COPY . .

RUN apt-get update && apt install -y libavutil-dev libavformat-dev libavfilter-dev libavdevice-dev libasound2-dev libogg-dev libopusfile-dev ffmpeg pkg-config llvm clang libpq-dev

RUN cargo build -p api-bin --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/api-bin .

RUN apt-get update && apt install -y libavutil-dev libavformat-dev libavfilter-dev libavdevice-dev libasound2-dev libogg-dev libopusfile-dev ffmpeg libpq-dev

EXPOSE 1451 1452 3478 3478/udp 50000-65535 50000-65535/udp


CMD ["./app-bin"]
