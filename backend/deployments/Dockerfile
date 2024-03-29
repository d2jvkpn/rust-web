#### stage 1
# FROM rust:slim-buster AS rust-web-backend_builder
FROM clux/muslrust:latest AS rust-web-backend_builder
LABEL stage=rust-web-backend_builder

ARG BUILD_Vendor=${BUILD_Vendor}
ENV SQLX_OFFLINE=true
ENV TZ=Asia/Shanghai
WORKDIR /opt/rust-web

COPY src         ./src
COPY static      ./static
COPY migrations  ./migrations
ADD Cargo.lock   Cargo.toml  sqlx-data.json ./
ADD src/_build_info.yaml  ./src/build_info.yaml

ADD deployments/cargo_config.toml deployments/cargo_vendor.toml ./
ADD vendor  ./vendor

RUN if [ "$BUILD_Vendor" = "true" ]; then \
    echo "==> BUILD_Vendor: true"; \
    sed -i 's#\(deb\|security\).debian.org#mirrors.aliyun.com#g' /etc/apt/sources.list; \
    mkdir -p /root/.cargo && mv cargo_config.toml /root/.cargo/config; \
    mkdir -p .cargo && mv cargo_vendor.toml .cargo/config.toml; \
  fi

RUN { \
    apt update && \
    apt -y upgrade && \
    apt install -y pkg-config libssl-dev ca-certificates && \
    apt clean; \
  } &> /dev/null

RUN rustup default nightly && \
  rustup target add x86_64-unknown-linux-musl && \
  cargo build --target x86_64-unknown-linux-musl --release

#### stage 2
FROM alpine:latest AS rust-web_runtime
ARG BUILD_Vendor=${BUILD_Vendor}
ENV TZ=Asia/Shanghai

RUN if [ "$BUILD_Vendor" = "true" ]; then \
    echo "==> BUILD_Vendor: true"; \
    sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories; \
  fi

RUN { \
    apk --no-cache update && \
    apk --no-cache upgrade && \
    apk --no-cache add tzdata ca-certificates; \
  } &> /dev/null

RUN adduser -D -u 1000 hello
USER hello
WORKDIR /home/hello/rust-web

COPY --from=rust-web-backend_builder \
  /opt/rust-web/target/x86_64-unknown-linux-musl/release/backend \
  ./backend

COPY migrations  ./migrations

EXPOSE 3010
# ENTRYPOINT ["./backend"]
# "--threads=0"
CMD ["./backend", "--config=configs/prod.yaml", "--addr=0.0.0.0", "--port=3010", "--release"]
