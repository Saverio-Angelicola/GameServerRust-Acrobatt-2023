FROM rust:latest

EXPOSE 5000

EXPOSE 5050

COPY . .

RUN cargo build --release

CMD ["/target/release/all-chimie-game-server", "5000"]
