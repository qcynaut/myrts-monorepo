FROM rust:1.74.0-slim-bullseye as builder

WORKDIR /app

COPY . .

RUN apt-get update && apt install -y libavutil-dev libavformat-dev libavfilter-dev libavdevice-dev libasound2-dev libogg-dev libopusfile-dev ffmpeg pkg-config llvm clang libpq-dev

RUN cargo build -p api-bin --release

FROM nginx:1.25.0-bullseye

WORKDIR /app

COPY --from=builder /app/target/release/api-bin .

COPY ./configs/nginx.conf /etc/nginx/conf.d/default.conf

COPY ./configs/run.sh .

RUN chmod +x ./run.sh

RUN apt-get update && apt install -y libavutil-dev libavformat-dev libavfilter-dev libavdevice-dev libasound2-dev libogg-dev libopusfile-dev ffmpeg libpq-dev

EXPOSE 80 1451 1452 3478 3478/udp 50000-65535 50000-65535/udp


CMD ["./run.sh"]
