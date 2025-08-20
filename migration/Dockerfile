FROM rust:latest

WORKDIR /usr/src/banshee-migration

COPY Cargo.lock Cargo.toml banned_users.txt /usr/src/banshee-migration/
COPY ./src /usr/src/banshee-migration/src

RUN cargo install --path .