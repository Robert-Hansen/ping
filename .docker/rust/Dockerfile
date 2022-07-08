FROM rust:1.62

## Add the wait script to the image
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait /wait
RUN chmod +x /wait

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

WORKDIR /usr/src/app

COPY . ./

EXPOSE 3000

VOLUME ["/usr/local/cargo"]

CMD /wait && diesel setup && cargo watch -x run