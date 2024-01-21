FROM rust:latest as build

WORKDIR /usr/src/DiscordBot

COPY . .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/DiscordBot/target/release/discord_bot_rust /usr/local/bin/discord_bot_rust

WORKDIR /usr/local/bin

CMD ["discord_bot_rust"]