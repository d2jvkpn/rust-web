#! make

# include envfile
# export $(shell sed 's/=.*//' envfile)

git_branch = $(shell git rev-parse --abbrev-ref HEAD)
git_time = $(shell git log -1 --format="%at" | xargs -I{} date -d @{} +%FT%T%:z)
build_time = $(shell date +'%FT%T%:z')
database_url = $(shell yq .database.dsn configs/local.yaml)
current = $(shell pwd)

install_sqlx:
	cargo add sqlx --features=runtime-actix-rustls,macros,postgres,uuid,chrono,migrate,offline,ipnetwork,json,decimal

init_db:
	docker run --name postgres_db -d \
	  -e TZ=Asia/Shanghai -e PGTZ=Asia/Shanghai \
	  -e POSTGRES_USER=postgres -e POSTGRES_DB=postgres -e POSTGRES_PASSWORD=secret \
	  -p 127.0.0.1:5432:5432 postgres:15-alpine
	# docker-compose -f docker-compose.yaml up -d
	docker exec -it postgres_db createuser --username=postgres hello --createdb --login
	docker exec -it postgres_db psql --username=postgres -c "ALTER ROLE hello PASSWORD 'world'"
	# docker exec -it postgres_db createdb --username=postgres --owner=hello rust_web
	# docker exec -it postgres_db createdb --username=hello --owner=hello rust_web
	sqlx database create
	# docker exec -it postgres_db dropdb --username=postgres rust_web
	export DATABASE_URL=postgres://hello:world@127.0.0.1:5432/rust_web > .env

connect_db:
	# without --password
	# docker exec -it postgres_db psql --username=postgres rust_web
	# psql --host=127.0.0.1 --port 5432 --username=hello rust_web
	docker exec -it postgres_db psql --username=hello rust_web

migrate:
	# sqlx migrate add create_users_table
	# sqlx migrate add alter_users_table
	# sqlx database create
	# sqlx database drop
	# sqlx database reset
	sqlx migrate run

prepare_data:
	cargo sqlx prepare -- --lib

cargo-vendor:
	cargo vendor --versioned-dirs

build-release:
	cargo build --release

docker-build:
	BuildLocal=true bash deployments/build.sh dev

run:
	cargo run
