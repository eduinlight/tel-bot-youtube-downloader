FROM rust:1.86.0-alpine3.21 AS base
RUN apk --update-cache add musl-dev pkgconfig openssl-dev clang-dev yt-dlp build-base

FROM base as build
ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /app
COPY src src
COPY Cargo.* .
RUN cargo build --release

FROM alpine as prod
RUN apk add --update --no-cache libgcc openssl-dev yt-dlp
COPY --from=build /app/target/release/tel-bot-youtube-downloader .
CMD ["./tel-bot-youtube-downloader"]
