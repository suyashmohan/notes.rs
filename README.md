This is a simple application written to learn the basics of Rust programming langauge.
It is a REST API project that use Actix-web along with PostgreSQL. 

# Setup Database
```
docker run -d -e POSTGRES_DB=rnote -e POSTGRES_USER=postgres -p 5432:5432 postgres
diesel migration run --database-url postgres://postgres@localhost/rnote
```

# How to Run
```
cargo run
```

# Testing

### Create

curl -XPOST -H "Content-type: application/json" -d '{ "title": "test", "body": "hello! world" }' 'http://127.0.0.1:8080/v1/notes' -v

### Read
curl -XGET -H "Content-type: application/json" 'http://127.0.0.1:8080/v1/notes/0ed59767-8c7a-4f32-871f-c2084111b957' -v

### Update
curl -XPOST -H "Content-type: application/json" -d '{ "title": "test", "body": "hello! again" }' 'http://127.0.0.1:8080/v1/notes/0ed59767-8c7a-4f32-871f-c2084111b957' -v


### Delete
```