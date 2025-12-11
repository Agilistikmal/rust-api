.PHONY: run

run:
	cargo run

docker-up:
	docker compose up -d

docker-down:
	docker compose down

docker-build:
	docker compose build

docker-rebuild:
	docker compose down
	docker compose build
	docker compose up -d