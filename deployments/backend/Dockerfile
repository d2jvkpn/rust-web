####
FROM rust:1-alpine AS rust-web-backend_builder

ARG BuildLocal=${BuildLocal}
ENV SQLX_OFFLINE=true
ENV TZ=Asia/Shanghai
WORKDIR /opt/rust-web

COPY backend/src         ./src
COPY backend/static      ./static
COPY backend/migrations  ./migrations
ADD backend/Cargo.lock   backend/Cargo.toml  backend/sqlx-data.json ./
ADD backend/src/_git-build-info.yaml  ./src/git-build-info.yaml

ADD deployments/config_cargo.toml deployments/config_vendor.toml ./
ADD backend/vendor  ./vendor

RUN if [ "$BuildLocal" = "true" ]; \
  then \
    sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories; \
    mkdir -p /root/.cargo && mv config_cargo.toml /root/.cargo/config; \
    mkdir -p .cargo && mv config_vendor.toml .cargo/config.toml; \
  fi

RUN { apk --no-cache update && \
    apk --no-cache upgrade; \
    apk --no-cache add tzdata musl-dev libressl-dev; \
  } &> /dev/null

RUN rustup default nightly && \
  rustup target add x86_64-unknown-linux-musl && \
  RUSTFLAGS='-C target-feature=+crt-static' \
  cargo build --target x86_64-unknown-linux-musl --release

####
FROM alpine:latest AS rust-web_runtime
ENV TZ=Asia/Shanghai

RUN if [ "$BuildLocal" = "true" ]; then \
    sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories; \
  fi

RUN { apk --no-cache update && \
    apk --no-cache upgrade; \
    apk --no-cache add tzdata ca-certificates; \
  } &> /dev/null

RUN adduser -D -u 1000 hello
USER hello
WORKDIR /home/hello/rust-web

COPY --from=rust-web-backend_builder \
  /opt/rust-web/target/x86_64-unknown-linux-musl/release/backend \
  ./backend

COPY backend/migrations  ./migrations

EXPOSE 3010
# ENTRYPOINT ["./backend"]
# "--threads=0"
CMD ["./backend", "--config=configs/prod.yaml", "--addr=0.0.0.0", "--port=3010", "--release"]
