init-dev:
	command -v nvm && exit 0
	curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
	nvm install --lts
	npm config set registry https://registry.npm.taobao.org && npm config get registry
	mkdir -p ~/Apps && npm set --location=global prefix ~/Apps
	npm install --global serve create-react-app yarn

init-project:
	# create-react-app frontend
	# cd frontend
	# npm install --save-dev env-cmd
	[ -f package-lock.json ] && rm package-lock.json
	yarn add env-cmd --dev
	yarn add sprintf-js antd
	# @aws-sdk/client-s3 ali-oss

install:
	yarn install

upgrade:
	yarn upgrade

run:
	npm run local

build-local:
	bash react-build.sh configs/local.env

docker-build:
	BuildLocal=true bash deployments/build.sh dev
