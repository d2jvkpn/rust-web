install_sqlx:
	cargo add sqlx --features=runtime-actix-rustls,macros,postgres,uuid,chrono,migrate,offline,ipnetwork

init_db:
	echo TODO

create_db:
	echo TODO
	sqlx database create

connect_db:
	# without --password
	docker exec -it postgres_db psql --username=hello --host=localhost rust_web

migrate:
	# sqlx migrate add create_users_table
	# sqlx migrate add alter_users_table
	# sqlx database create
	# sqlx database drop
	# sqlx database reset
	sqlx migrate run

prepare_data:
	cargo sqlx prepare -- --lib

release:
	cargo build --release

run:
	cargo run

cargo-vendor:
	cargo vendor --versioned-dirs
