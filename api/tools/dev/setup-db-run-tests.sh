psql postgres://rustyindie:rustyindie@localhost:5433/rustyindie -f ./tools/dev/clean.sql
psql postgres://rustyindie:rustyindie@localhost:5433/rustyindie -f ./tools/dev/setup-dev-data.sql
cargo test -- --nocapture