FROM debian:bullseye-20231120-slim as downloader

RUN apt-get update && apt-get install -y curl

RUN curl -L https://github.com/yudai/gotty/releases/download/v1.0.1/gotty_linux_amd64.tar.gz -o gotty.tar.gz
RUN tar xf gotty.tar.gz && mv gotty /usr/local/bin

FROM debian:bullseye-20231120-slim

COPY --from=downloader /usr/local/bin/gotty /usr/local/bin

EXPOSE 8080

CMD ["gotty", "-w", "-p", "8080", "-a", "0.0.0.0", "/bin/bash"]