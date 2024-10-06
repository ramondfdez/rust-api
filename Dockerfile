FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN cargo build --release
RUN strip target/release/rust-api

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/rust-api .

EXPOSE 8000

CMD [ "./rust-api" ]