mig-create:
	diesel migration generate --diff-schema=src/tables.rs $(name)

mig-run:
	diesel migration run

run:
	cargo run

run-auto:
	cargo watch -x run

format:
	cargo clippy
	cargo fmt

secret:
	openssl rand -base64 32

