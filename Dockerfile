### Build image
FROM alpine as rust-build
RUN apk add rust cargo --no-cache

FROM rust-build as app-build
WORKDIR /app
RUN apk add --no-cache openssl-dev libgcc nasm
COPY . .
RUN cargo build --release


### application image
FROM alpine as imaginary
WORKDIR /app
RUN apk add --no-cache openssl-dev libgcc
COPY --from=app-build /app/target/release/imaginary .
CMD ["/app/imaginary"]
