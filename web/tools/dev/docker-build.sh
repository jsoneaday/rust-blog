echo "build web server with nginx"
# docker compose -f dev.yml -p rustyindie-web up -d --build
docker build -f dev-dockerfile -t rustyindie-web-i .
docker run --name rustyindie-web-c -d -p 80:80 rustyindie-web-i