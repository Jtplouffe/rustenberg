FROM rust:1.67 as build

WORKDIR /server

RUN cargo init --bin

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm -rf ./src

COPY . .

RUN rm ./target/release/deps/rustenberg*
RUN cargo build --release

# ---

FROM debian

RUN apt update && apt install chromium tini -y

RUN \
    groupadd --gid "1001" rustenberg &&\
    useradd --uid "1001" --gid rustenberg --shell /bin/bash --home /home/rustenberg --no-create-home rustenberg &&\
    mkdir /home/rustenberg && \
    chown rustenberg: /home/rustenberg

COPY --from=build /server/target/release/rustenberg /home/rustenberg

USER rustenberg

WORKDIR /home/rustenberg

EXPOSE 8000
ENTRYPOINT ["/usr/bin/tini"]
CMD ["./rustenberg"]
