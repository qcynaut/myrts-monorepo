main() {
    apt update -y && sudo apt upgrade -y
    apt install -y curl pulseaudio libpulse-dev librust-libpulse-sys-dev alsa-utils libogg-dev libvorbis-dev ffmpeg libopus-dev libasound2-dev libudev-dev libopusfile-dev llvm-dev libclang-dev clang openssl libssl-dev libavutil-dev libavcodec-dev libavformat-dev libavfilter-dev libavdevice-dev ffmpeg
    curl -L https://api.myrts.id/install/myrts-client -o /tmp/myrts-client
    chmod +x /tmp/myrts-client
    mv /tmp/myrts-client /var/lib/myrts/
    systemctl restart myrts-client && systemctl enable myrts-client
}

main