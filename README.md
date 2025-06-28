# Auth API Example

Simple example showing how to integrate the user-auth-lib for authentication.

## Quick Start

1. **Start the database:**
   ```bash
   docker-compose up -d
   ```

2. **Run the API:**
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:3000`

## API Endpoints

### Public Endpoints
- `POST /register` - Register a new user
- `POST /login` - Login and get JWT token

### Protected Endpoints (require Bearer token)
- `GET /me` - Get current user info
- `GET /users/:id` - Get user by ID
- `PATCH /users/:id` - Update user (only own profile)

## Example Usage

### Register a user:
```bash
curl -X POST http://localhost:3000/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "display_name": "John Doe"
  }'
```

### Login:
```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

### Get current user (with token):
```bash
curl -X GET http://localhost:3000/me \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Environment Variables

Copy `.env.example` to `.env` and adjust as needed:
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key for JWT signing
- `PORT` - Server port (default: 3000)