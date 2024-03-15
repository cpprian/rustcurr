FROM messense/rust-musl-cross:x86_64-musl

ENV APP_NAME=rustcurr

WORKDIR /app

COPY Cargo.toml .env src ./

RUN rustup update beta && \
    rustup target add --toolchain beta x86_64-unknown-linux-musl
RUN cargo build --release --target x86-64-unknown-linux-musl
RUN cp ./target/release/${APP_NAME} /bin/${APP_NAME}

CMD ["/bin/rustcurr -b PLN -t EUR -a 1234"]