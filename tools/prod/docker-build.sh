echo "build web server with nginx"
# docker compose -f dev.yml -p rustyindie-web up -d --build
docker build -f prod-dockerfile -t rustyindie-web-i .
docker run --name rustyindie-web-c -d -p 80:80 -p 443:443 -p 3001:3001 rustyindie-web-i