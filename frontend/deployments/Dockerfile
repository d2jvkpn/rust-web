# Dockerfile

#### stage 1
FROM node:lts-alpine AS builder
LABEL stage=rust-web-frontend_builder

ARG ENV_File=${ENV_File}
ARG REACT_APP_BuildTime=${REACT_APP_BuildTime}
ENV TZ=Asia/Shanghai

# RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories
# RUN { apk --no-cache update && apk --no-cache upgrade && apk --no-cache add tzdata; } &> /dev/null

WORKDIR /opt/rust-web-frontend
ADD *.json  react-build.sh yarn.lock ./
COPY  configs/  ./configs
COPY  public/   ./public
COPY  src/      ./src

# RUN npm install --global yarn --registry=https://registry.npm.taobao.org
RUN yarn install --registry=https://registry.npm.taobao.org &> /dev/null
RUN sh react-build.sh $ENV_File # bash isn't installed in apline in default

#### stage 2
FROM nginx:stable-alpine

WORKDIR /opt/nginx-web
# COPY /opt/rust-web-frontend/build  /usr/share/nginx/html
COPY --from=builder /opt/rust-web-frontend/build ./

# events {
# 	worker_connections  1024;
# }
RUN sed -i '/worker_processes/s#auto#4#' /etc/nginx/nginx.conf
RUN mv /etc/nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf.bk
COPY deployments/nginx-web.conf /etc/nginx/conf.d/nginx-web.conf

EXPOSE 80