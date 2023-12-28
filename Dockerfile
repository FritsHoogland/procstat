FROM rust:1.74.1

WORKDIR /usr/src/procstat
COPY . .

RUN cargo build
