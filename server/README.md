# FinStack API Server

A RESTful API server built with Actix Web and Diesel ORM.

## Setup

1. Make sure you have Docker and Docker Compose installed

2. Create a `.env` file with the following contents:

```env
DATABASE_URL=postgres://user:passw0rd@localhost:5432/finstack
SERVER_URL=127.0.0.1:8080
RUST_LOG=info
```

3.Start the PostgreSQL database with Docker:

```bash
docker-compose up -d
```

4.Run the API server:

```bash
cargo run
```

## API Endpoints

- `GET /api/users` - Get all users
- `POST /api/users` - Create a new user
- `GET /api/users/{userId}` - Get a specific user
- `PATCH /api/users/{userId}` - Update a user
- `DELETE /api/users/{userId}` - Delete a user

## User Model

- `id`: UUID
- `first_name`: String (max 50 chars)
- `last_name`: String (max 50 chars)
- `email`: String (unique)
- `password`: String
- `created_at`: Timestamp
- `updated_at`: Timestamp
