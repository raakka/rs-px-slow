FROM rust:1.47.0

WORKDIR usr/src/pxapi
COPY . .

EXPOSE 8080

RUN cargo install --path .
#RUN cargo run --release

CMD ["rs-px-slow"]
