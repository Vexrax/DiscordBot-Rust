FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
WORKDIR /DiscordBot-Rust

FROM scratch
COPY --from=builder /DiscordBot-Rust/target/x86_64-unknown-linux-musl/release/DiscordBot-Rust /DiscordBot-Rust
ENTRYPOINT ["/main"]
EXPOSE 3000