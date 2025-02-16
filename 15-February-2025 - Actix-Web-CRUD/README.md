#### User Management CRUD App

Minimal Identity CRUD REST application featuring actix-web framework

## Features

- [x] Telemetry
- [x] Create a user
- [x] Read users
- [x] Database: In-memory

- [] Read a user
- [] Update a user
- [] Delete a user

## Usage

```bash
$ cargo run
```

## Endpoints

- `GET /users` - List all users
- `POST /users` - Create a user

- `PUT /users/{id}` - Update a user by id
- `DELETE /users/{id}` - Delete a user by id

- `GET /status` - Health check

## Testing

#### Create a user

```bash
$ curl -X POST -H "Content-Type: application/json" -d '{"username": "john", "password": "123456", "email": "johndoe@xyz.com"}' http://localhost:5000/users
```

#### List all users

```bash
$ curl -X GET http://localhost:5000/users
```
