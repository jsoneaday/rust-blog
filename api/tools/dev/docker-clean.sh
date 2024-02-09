rm -rf dbdata

docker compose -f dev.yml -p rustyindie-api down -v

docker rmi rustyindie-api postgres:14-alpine