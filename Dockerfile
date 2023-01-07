FROM rust:alpine

WORKDIR /app


COPY . .
RUN apk update && apk add musl-dev opus cmake make
RUN sed -i 's/.context("Failed to load `.env` file")?;/.ok();/' src/main.rs

RUN cargo build --release



FROM alpine:latest

WORKDIR /app

RUN apk update && apk add opus yt-dlp ffmpeg

COPY --from=0 /app/target/release/codify /app


CMD [ "./codify" ]