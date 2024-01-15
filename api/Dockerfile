FROM rust:1.75 as builder
WORKDIR /usr/src/rust-blog
COPY . .
RUN cargo install --debug --path .
 
FROM debian:bullseye
RUN apt-get update
RUN apt-get install -y wget
RUN apt-get install -y build-essential
COPY --from=builder /usr/src/rust-blog /usr/local/bin/rust-blog
WORKDIR /usr/local/bin/rust-blog
ENTRYPOINT [ "./target/debug/rust-blog-api" ]