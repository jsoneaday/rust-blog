rm -rf dbdata

docker compose -f prod.yml -p rustyindie-api-p down -v

docker rmi rustyindie-api-i postgres:14-alpine