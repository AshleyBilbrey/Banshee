FROM lukemathwalker/cargo-chef:latest-rust-1.84.1 AS chef
WORKDIR /banshee


FROM chef AS planner
COPY Cargo.lock Cargo.toml /banshee/
COPY ./src/ /banshee/src/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /banshee/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY Cargo.lock Cargo.toml /banshee/
COPY ./src /banshee/src
RUN cargo build --release --bin banshee

FROM debian:bookworm-slim AS runtime
WORKDIR /banshee
COPY --from=builder /banshee/target/release/banshee /usr/local/bin
ENTRYPOINT ["/usr/local/bin/banshee"]