FROM rust:alpine AS build
WORKDIR /app

RUN apk add pkgconfig openssl-dev libc-dev

COPY . .
RUN cargo install --path .

FROM alpine:latest

WORKDIR /app
RUN apk update \
    && apk add openssl ca-certificates

EXPOSE 3000
COPY --from=build /app/target/release/pfe_backend /app/pfe_backend 
CMD ["/app/pfe_backend"]
