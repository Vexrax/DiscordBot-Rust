FROM rust:latest as build

WORKDIR /usr/src/DiscordBot

COPY . .

RUN apt update
RUN apt upgrade
RUN apt-get update && apt install -y openssl
RUN cargo build --release

FROM rust:latest
COPY --from=build /usr/src/DiscordBot/target/release/discord_bot_rust /usr/local/bin/discord_bot_rust

WORKDIR /usr/local/bin

CMD ["discord_bot_rust"]