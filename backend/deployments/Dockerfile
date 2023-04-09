####
FROM rust:1-slim-buster as rust-web-backend_builder

ENV SQLX_OFFLINE=true
ENV TZ=Asia/Shanghai
WORKDIR /opt/rust-web-backend

RUN mkdir -p /root/.cargo
ADD Cargo.lock Cargo.toml sqlx-data.json ./
ADD .git-build-info.yaml ./src/git-build-info.yaml
# ADD deployments/cargo_config.toml /root/.cargo/config
COPY src ./src
COPY static ./static
COPY migrations ./migrations

# RUN sed -i 's#http://\(deb\|security\).debian.org#https://mirrors.aliyun.com#g' /etc/apt/sources.list

RUN apt -y update && apt -y upgrade && apt install -y musl-tools
# tzdata is already the newest version

RUN rustup target add x86_64-unknown-linux-musl && \
  cargo build --target x86_64-unknown-linux-musl --release && \
  mv target/x86_64-unknown-linux-musl/release/backend target/

####
FROM alpine:latest AS rust-web_runtime
ENV TZ=Asia/Shanghai
WORKDIR /opt/rust-web-backend

COPY --from=rust-web-backend_builder /opt/rust-web-backend/target/backend ./backend

EXPOSE 3010
# ENTRYPOINT ["./backend"]
#  "--threads=0"
CMD ["./backend", "--config=configs/prod.yaml", "--addr=0.0.0.0", "--port=3010", "--release"]