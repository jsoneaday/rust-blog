psql postgres://rustblog:rustblog@localhost:5433/rustblog -f ./tools/clean.sql
cargo test -- --nocapture