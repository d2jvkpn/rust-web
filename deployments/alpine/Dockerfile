####
FROM rust:1-alpine as rust-web_builder

ENV SQLX_OFFLINE=true TZ=Asia/Shanghai
WORKDIR /opt/rust-web

RUN mkdir -p /root/.cargo
ADD deployments/cargo_config.toml /root/.cargo/config
ADD Cargo.lock Cargo.toml sqlx-data.json ./
ADD .git-build-info.yaml ./src/git-build-info.yaml
COPY src ./src
COPY static ./static
COPY migrations ./migrations

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories; \
  { apk --no-cache update && apk --no-cache upgrade; apk add tzdata musl-dev; } &> /dev/null

RUN cargo build --release

####
FROM alpine:latest AS rust-web_runtime
ENV TZ=Asia/Shanghai
WORKDIR /opt/rust-web

COPY --from=rust-web_builder /opt/rust-web/target/release/rust-web /opt/rust-web/rust-web

EXPOSE 3000
# ENTRYPOINT ["./rust-web"]
# "--threads=0"
CMD ["./rust-web", "--config=configs/prod.yaml", "--addr=0.0.0.0", "--port=3000", "--release"]
