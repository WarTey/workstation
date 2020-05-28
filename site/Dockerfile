FROM ekidd/rust-musl-builder:nightly-2020-05-07 AS builder
ADD --chown=rust:rust . .
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
RUN apk --no-cache add bash
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/site /
COPY --from=builder /home/rust/src/.env /
COPY --from=builder /home/rust/src/static /static
COPY --from=builder /home/rust/src/templates /templates
COPY --from=builder /home/rust/src/wait-for-it.sh /
EXPOSE 8000
