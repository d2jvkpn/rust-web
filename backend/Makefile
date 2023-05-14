install-sqlx:
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
	# database create
	# database drop
	# database reset
	sqlx migrate run

release:
	cargo build --release

run:
	cargo run
