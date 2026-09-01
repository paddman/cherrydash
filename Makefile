.PHONY: check fmt test web-build compose-config compose-up compose-down

check:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cargo test --workspace

fmt:
	cargo fmt --all

test:
	cargo test --workspace

web-build:
	cd web && npm install --no-audit --no-fund && npm run build

compose-config:
	docker compose -f deploy/compose/docker-compose.yml config --quiet

compose-up:
	docker compose -f deploy/compose/docker-compose.yml up --build

compose-down:
	docker compose -f deploy/compose/docker-compose.yml down
