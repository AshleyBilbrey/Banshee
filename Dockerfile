FROM rust:latest

WORKDIR /usr/src/banshee

COPY Cargo.lock Cargo.toml /usr/src/banshee/
COPY ./src /usr/src/banshee/src

RUN cargo install --path .

ENTRYPOINT ["banshee"]