if [ -z "$1" ]; then
    echo "Error missing db username"
    exit 1
fi
if [ -z "$2" ]; then
    echo "Error missing db password"
    exit 1
fi

echo "build new images"
docker compose -f prod.yml -p rustyindie-api-p up -d --build
echo "wait for db setup to complete"
sleep 2

echo "run sqlx migrations"
sqlx database create --database-url "postgres://$1:$2@localhost:5433/rustyindie"
sqlx migrate run --database-url "postgres://$1:$2@localhost:5433/rustyindie"
