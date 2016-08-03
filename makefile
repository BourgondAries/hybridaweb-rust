all:
	cargo build
	# bash -c 'SLOG_LEVEL=Trace ./target/debug/hybridaweb'
dev:
	cargo build --features dev
	# bash -c 'SLOG_LEVEL=Trace ./target/debug/hybridaweb'
fmto:
	cargo fmt -- --write-mode overwrite
