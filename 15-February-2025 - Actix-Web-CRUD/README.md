#### User Management CRUD App

Minimal Identity CRUD REST application featuring actix-web framework

## Features

- [x] Telemetry
- [x] Health check
- [x] Input validation
- [x] Create a user
- [x] Read users
- [x] Read a user
- [x] Database: In-memory

- [] Update a user
- [x] Delete a user

## Usage

```bash
$ cargo run
```

## Endpoints

- `GET /users` - List all users
- `POST /users` - Create a user
- `PUT /users/{username}` - Update a user by username
- `DELETE /users/{username}` - Delete a user by username

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

#### Read a user

```bash
$ curl -X GET http://localhost:5000/users/john
```

#### Delete a user

```bash
$ curl -X DELETE http://localhost:5000/users/john
```

## License

MIT

```

```
