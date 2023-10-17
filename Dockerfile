### Build image
FROM alpine as rust-build
RUN apk add rust cargo --no-cache

FROM rust-build as app-build
WORKDIR /app
COPY . .
RUN cargo build --release


### Runner image
FROM alpine as rust-actix-api-example
WORKDIR /app
COPY --from=app-build /app/target/release/imaginary .
CMD ["/app/imaginary"]