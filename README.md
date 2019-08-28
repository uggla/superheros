# Rust superheros api

Demo API written in Rust with Actix-web and Diesel.<br/>
A simple Continuous Integration was configured as an example.
<br/>
<br/>
[![Build Status](https://travis-ci.org/uggla/superheros.svg?branch=master)](https://travis-ci.org/uggla/superheros)

## Managing the application

All commands must be run from the root of the project.

### Run the application
```
docker-compose up -d
```

### Stop the application
```
docker-compose down
```

### Rebuild the application
```
docker-compose up -d --build
```
### Integration tests
```
cd robotframework && docker run --rm -v $(pwd)/reports:/opt/robotframework/reports:Z -v $(pwd)/tests:/opt/robotframework/tests:Z -e BROWSER=firefox --shm-size=2g --network=host ppodgorsek/robot-framework:latest
```

## Local setup to get a faster feedback loop

Rust and Cargo must be installed. The Postgres database will be run using a docker container.

### Create a local postgres db:
```
echo "DATABASE_URL=postgres://uggla:superheros@localhost/superheros" >.env
docker run -d -p 5432:5432 -v $PWD/data:/tmp/data -e POSTGRES_USER=uggla -e POSTGRES_PASSWORD=superheros --name superheros postgres
diesel setup
```

### Populate DB
```
diesel migration run
```
### Building the application

System must be connected to the internet in order to get the required crates (packages). The compilation can be long depending on system configuration.

Debug version
```
cargo build
```
Release version
```
cargo build --release
```
### Running the application

Debug version
```
cargo run
```
Release version
```
cargo run --release
```
