FROM rust:slim-bullseye as builder

# setting up working dir
WORKDIR /var/www

# install SQLite3 libs for diesel compat
RUN apt-get update && \
    apt-get install -y sqlite3 libsqlite3-dev && \
    rm -rf /var/lib/apt/lists/*

# installing diesel cli tools
RUN cargo install diesel_cli --no-default-features --features sqlite