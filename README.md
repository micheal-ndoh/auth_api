# auth_api

# Database Setup

To run a local Postgres database for development:

```
docker run --name auth-postgres -e POSTGRES_PASSWORD=yourpassword -e POSTGRES_USER=authuser -e POSTGRES_DB=authdb -p 5432:5432 -d postgres
```

Set your `.env` file:

```
DATABASE_URL=postgres://authuser:yourpassword@localhost:5432/authdb
JWT_SECRET=your_jwt_secret
```

Run migrations to create the tables:

```
# Connect to the database
psql -h localhost -U authuser -d authdb

# Then run:
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    firstname VARCHAR(255) NOT NULL,
    lastname VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    token VARCHAR(512) NOT NULL,
    expires_at TIMESTAMP NOT NULL
);
```
