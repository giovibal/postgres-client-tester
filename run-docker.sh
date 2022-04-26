#!/usr/bin/env bash

docker run -it --rm \
  --net=host \
  -e "RUST_LOG=info" \
  -e "DB_CONN=postgresql://postgres:Passw0rd@localhost:5432/tests" \
  giovibal/postgres-client-tester --sleep 500
