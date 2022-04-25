# Postgres Client Tester

A postgres client to test connectivity and query execution against a postgres server.

Use to evaluate ha of writers and readers.



## Quick start

1. Clone this repo


    git clone ...
    

2. start a local postgres with doker


    ./start-postgres.sh


3. start inserting records ...


    ./run-insert-loop.sh


4. start reading records ...


    ./run-select-loop.sh


