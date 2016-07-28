all:
	cargo build --features dev
	bash -c 'SLOG_LEVEL=Trace ./target/debug/hybridaweb'
