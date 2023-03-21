####
FROM rust:1-slim-buster as rust-web_builder

ENV SQLX_OFFLINE=true TZ=Asia/Shanghai
WORKDIR /opt/rust-web

RUN mkdir -p /root/.cargo
ADD deployments/cargo_config.toml /root/.cargo/config
ADD Cargo.lock Cargo.toml sqlx-data.json ./
ADD .git-build-info.yaml ./src/git-build-info.yaml
COPY src ./src
COPY static ./static
COPY migrations ./migrations

RUN sed -i 's#http://\(deb\|security\).debian.org#https://mirrors.aliyun.com#g' \
  /etc/apt/sources.list; apt -y update && apt -y upgrade && apt install -y musl-tools
# tzdata is already the newest version

RUN rustup target add x86_64-unknown-linux-musl && \
  cargo build --target x86_64-unknown-linux-musl --release

####
FROM alpine:latest AS rust-web_runtime
ENV TZ=Asia/Shanghai
WORKDIR /opt/rust-web

COPY --from=rust-web_builder /opt/rust-web/target/x86_64-unknown-linux-musl/release/rust-web \
  /opt/rust-web/rust-web

EXPOSE 3000
# ENTRYPOINT ["./rust-web"]
#  "--threads=0"
CMD ["./rust-web", "--config=configs/prod.yaml", "--addr=0.0.0.0", "--port=3000", "--release"]
