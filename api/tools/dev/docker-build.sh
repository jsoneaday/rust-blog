echo "build new images"
docker compose -f dev.yml -p rustyindie-api up -d --build
echo "wait for db setup to complete"
sleep 10

echo "run sqlx migrations"
sqlx database create --database-url postgres://rustyindie:rustyindie@localhost:5433/rustyindie
sqlx migrate run --database-url postgres://rustyindie:rustyindie@localhost:5433/rustyindie

echo "setup test data"
psql postgres://rustyindie:rustyindie@localhost:5433/rustyindie -f ./tools/dev/setup-dev-data.sql
