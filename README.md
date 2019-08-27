# Rust superheros api

All command must be run from the root of the project

## Run the application
```
docker-compose up -d
```

## Stop the application
```
docker-compose down
```

## Rebuild the application
```
docker-compose up -d --build
```

## Create a local postgres db:
```
echo "DATABASE_URL=postgres://uggla:superheros@localhost/superheros" >.env
docker run -d -p 5432:5432 -v $PWD/data:/tmp/data -e POSTGRES_USER=uggla -e POSTGRES_PASSWORD=superheros --name superheros postgres
diesel setup
```

## Populate DB
```
diesel migration run
```

## Integration tests
```
cd robotframework && docker run --rm -v $(pwd)/reports:/opt/robotframework/reports:Z -v $(pwd)/tests:/opt/robotframework/tests:Z -e BROWSER=firefox --shm-size=2g --network=host ppodgorsek/robot-framework:latest
```
