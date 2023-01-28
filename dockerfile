FROM rust:latest as builder
WORKDIR /usr/src/anonymous_bot
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
WORKDIR /home
RUN apt-get update
RUN apt-get -y install wget
RUN apt-get -y install screen
RUN rm -rf /var/lib/apt/lists/*
RUN wget -qO gotty.tar.gz https://github.com/yudai/gotty/releases/latest/download/gotty_linux_amd64.tar.gz && tar xf gotty.tar.gz -C /usr/local/bin && rm -rf gotty.tar.gz
COPY --from=builder /usr/local/cargo/bin/anonymous_bot /usr/local/bin/anonymous_bot
COPY ./start.sh .
ENV DISCORD_TOKEN=""
ENV CHANNEL_ID=""
RUN chmod 777 /home/start.sh

ENTRYPOINT ["/home/start.sh"]

EXPOSE 8080/tcp