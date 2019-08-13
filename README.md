# Rust superheros api

## Create postgres db:
```
docker run -d -p 5432:5432 -v $PWD/data:/tmp/data -e POSTGRES_USER=uggla -e POSTGRES_PASSWORD=superheros --name superheros postgres
diesel setup
```

## Populate DB
```
diesel migration run
```
