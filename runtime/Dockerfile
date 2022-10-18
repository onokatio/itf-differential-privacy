FROM rust:1.64-slim as builder

WORKDIR /usr/src/myapp
COPY . .
RUN cargo build
RUN ls

FROM rust:1.64-alpine

COPY --from=builder /usr/src/myapp/target/debug/runtime /usr/local/bin/runtime
CMD ["runtime"]
