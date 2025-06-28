# Auth API Example

This is a simple example implementation for the [user-mgmt-api](https://github.com/AncientiCe/user-mgmt-api) Rust library.

It shows how to integrate the plug-and-play library for authentication and user management with PostgreSQL, including all registration, login, JWT, and profile endpoints.

---

## Quick Start

1. **Start the database:**
   ```bash
   docker-compose up -d
   ```

2. **Run the API:**
   ```bash
   cargo run
   ```

The API will be available at [http://localhost:3000](http://localhost:3000)

---

## API Endpoints

### Public
- `POST /register` — Register a new user
- `POST /login` — Login and receive a JWT token

### Protected (require Bearer token)
- `GET /me` — Get current user info
- `GET /users/:id` — Get user by ID
- `PATCH /users/:id` — Update your display name

---

## Example Usage

### Register a user
```bash
curl -X POST http://localhost:3000/register   -H "Content-Type: application/json"   -d '{
    "email": "user@example.com",
    "password": "password123",
    "display_name": "John Doe"
  }'
```

### Login
```bash
curl -X POST http://localhost:3000/login   -H "Content-Type: application/json"   -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

### Get current user (with JWT token)
```bash
curl -X GET http://localhost:3000/me   -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

---

## Environment Variables

Copy and edit `.env` as needed:

- `DATABASE_URL` — PostgreSQL connection string
- `JWT_SECRET` — Secret key for JWT signing
- `PORT` — Server port (default: 3000)

---

## Reference & Testing

- **Library source:** [user-mgmt-api](https://github.com/AncientiCe/user-mgmt-api)
- **API reference/Postman collection:** [postman.json](https://github.com/AncientiCe/user-mgmt-api/blob/main/postman.json)

---

## License

MIT

---

_This is just an example implementation. For full details, see the main library._
