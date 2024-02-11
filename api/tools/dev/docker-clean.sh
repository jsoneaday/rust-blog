rm -rf dbdata

docker compose -f dev.yml -p rustyindie-api-p down -v

docker rmi rustyindie-api-i postgres:14-alpine