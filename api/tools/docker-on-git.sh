#!/bin/bash

rm -rf devdb
docker compose down --rmi 'all' --remove-orphans
docker compose up -d --build