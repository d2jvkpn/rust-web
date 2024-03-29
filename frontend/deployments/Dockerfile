#### stage 1
FROM node:lts-alpine AS rust-web-frontend_builder
LABEL stage=rust-web-frontend_builder

ARG ENV_File=${ENV_File}
ARG REACT_APP_BuildTime=${REACT_APP_BuildTime}
ARG BuildLocal=${BuildLocal}
ENV TZ=Asia/Shanghai

WORKDIR /opt/rust-web
ADD *.json react-build.sh yarn.lock ./
COPY configs/ ./configs
COPY public/ ./public
COPY src/ ./src
RUN mkdir logs

RUN if [ "$BuildLocal" = "true" ]; then \
    echo "==> BuildLocal: true"; \
    sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories; \
    npm config set registry https://registry.npm.taobao.org; \
  fi

# RUN { apk --no-cache update && apk --no-cache upgrade && apk --no-cache add tzdata; } &> /dev/null
# RUN npm install --global yarn --registry=https://registry.npm.taobao.org
# RUN yarn install --registry=https://registry.npm.taobao.org &> /dev/null

RUN yarn install &> /dev/null
RUN sh react-build.sh $ENV_File # bash isn't installed in apline in default

#### stage 2
FROM nginx:stable-alpine

ARG BuildLocal=${BuildLocal}
ENV TZ=Asia/Shanghai

WORKDIR /opt/nginx-web
# COPY /opt/rust-web/build  /usr/share/nginx/html
COPY --from=rust-web-frontend_builder /opt/rust-web/build ./

# events {
# 	worker_connections  1024;
# }
RUN sed -i '/worker_processes/s#auto#4#' /etc/nginx/nginx.conf
RUN mv /etc/nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf.bk
COPY deployments/nginx-web.conf /etc/nginx/conf.d/nginx-web.conf

EXPOSE 80
