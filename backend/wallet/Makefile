SHELL = /bin/bash

WALLET_DATABASE_URL ?= postgres://postgres:postgres@localhost/wallet
WALLET_TEST_DATABASE_URL ?= postgres://postgres:postgres@localhost/wallet_test

.PHONY: all
all: fmt lint test

.PHONY: fmt
fmt:
	@cargo fmt --all

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: test
test: db_test
	@cargo test --workspace

.PHONY: db
db:
	@diesel --database-url ${WALLET_DATABASE_URL} setup
	@diesel --database-url ${WALLET_DATABASE_URL} migration run
	@diesel --database-url ${WALLET_DATABASE_URL} migration list

.PHONY: db_test
db_test:
	@diesel --database-url ${WALLET_TEST_DATABASE_URL} setup
	@diesel --database-url ${WALLET_TEST_DATABASE_URL} migration run
	@diesel --database-url ${WALLET_TEST_DATABASE_URL} migration list

.PHONY: run
run:
	@cargo run

.PHONY: grpc
grpc:
	@grpcurl -plaintext -proto ../../api/osaifu/wallet/v1/wallet.proto -d '{"owner": "kzmake"}' localhost:50051 osaifu.wallet.v1.WalletService/Create
	@grpcurl -plaintext -proto ../../api/osaifu/wallet/v1/wallet.proto -d '{"id": "0123456789ABCDEFGHJKMNPQRSTVWXYZ"}' localhost:50051 osaifu.wallet.v1.WalletService/Get
	@grpcurl -plaintext -proto ../../api/osaifu/wallet/v1/wallet.proto -d '{"id": "0123456789ABCDEFGHJKMNPQRSTVWXYZ"}' localhost:50051 osaifu.wallet.v1.WalletService/Delete
