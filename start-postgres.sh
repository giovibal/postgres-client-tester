#!/usr/bin/env bash

docker run \
    --rm \
    --name=postgres \
    -p 5432:5432 \
    -e POSTGRES_PASSWORD=Passw0rd \
    -v postgres12-data:/var/lib/postgresql/data \
    --rm \
    -it \
    postgres:12
