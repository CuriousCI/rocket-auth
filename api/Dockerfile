# syntax=docker/dockerfile:1
FROM rustlang/rust:nightly-bullseye AS builder

WORKDIR /app 

RUN apt-get update
RUN apt-get install libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./ . 
RUN cargo build --release


FROM debian:bullseye

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/diesel ./
COPY --from=builder /app/target/release/cookies ./


COPY ./migrations ./migrations
COPY ./public ./public

COPY ./.env ./
COPY ./Rocket.toml ./
COPY ./run.sh ./

RUN apt-get update
RUN apt-get install libpq-dev -y
RUN apt-get install iproute2 -y

EXPOSE 8000

CMD [ "./run.sh" ]